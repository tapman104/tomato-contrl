#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::VecDeque;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use eframe::egui;
use remctrl::server::run_server;
use remctrl::transport::ConnectionState;

enum AppStatus {
    Idle,
    Listening { local_ip: String },
    Connected { peer_addr: String },
    Error { message: String },
}

struct RemCtrlApp {
    status: Arc<Mutex<AppStatus>>,
    event_log: Arc<Mutex<VecDeque<String>>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl RemCtrlApp {
    fn new(runtime: Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            status: Arc::new(Mutex::new(AppStatus::Idle)),
            event_log: Arc::new(Mutex::new(VecDeque::new())),
            server_handle: None,
            runtime,
        }
    }

    fn get_local_ip() -> String {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(_) => return "Unknown".to_string(),
        };
        match socket.connect("8.8.8.8:80") {
            Ok(_) => match socket.local_addr() {
                Ok(addr) => addr.ip().to_string(),
                Err(_) => "Unknown".to_string(),
            },
            Err(_) => "Unknown".to_string(),
        }
    }

    fn start_server(&mut self, ctx: egui::Context) {
        let status_clone = self.status.clone();
        let log_clone = self.event_log.clone();
        let local_ip = Self::get_local_ip();

        *self.status.lock().unwrap_or_else(|e| e.into_inner()) = AppStatus::Listening {
            local_ip: local_ip.clone(),
        };

        let handle = self.runtime.spawn(async move {
            let ctx_for_state = ctx.clone();
            let ctx_for_event = ctx.clone();
            let status_for_state = status_clone.clone();

            let res = run_server(
                move |state| {
                    let mut s = status_for_state.lock().unwrap_or_else(|e| e.into_inner());
                    match state {
                        ConnectionState::Listening => {
                            *s = AppStatus::Listening {
                                local_ip: local_ip.clone(),
                            };
                        }
                        ConnectionState::Connected { peer_addr } => {
                            *s = AppStatus::Connected { peer_addr };
                        }
                        ConnectionState::Disconnected => {
                            *s = AppStatus::Idle;
                        }
                    }
                    ctx_for_state.request_repaint();
                },
                move |event| {
                    let mut log = log_clone.lock().unwrap_or_else(|e| e.into_inner());
                    let json = serde_json::to_string(event).unwrap_or_else(|_| "{}".to_string());
                    log.push_back(json);
                    if log.len() > 30 {
                        log.pop_front();
                    }
                    ctx_for_event.request_repaint();
                },
            )
            .await;

            if let Err(e) = res {
                *status_clone.lock().unwrap_or_else(|e| e.into_inner()) = AppStatus::Error {
                    message: e.to_string(),
                };
                ctx.request_repaint();
            } else {
                *status_clone.lock().unwrap_or_else(|e| e.into_inner()) = AppStatus::Idle;
                ctx.request_repaint();
            }
        });

        self.server_handle = Some(handle);
    }

    fn stop_server(&mut self) {
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
        *self.status.lock().unwrap_or_else(|e| e.into_inner()) = AppStatus::Idle;
        self.event_log.lock().unwrap_or_else(|e| e.into_inner()).clear();
    }
}

impl Drop for RemCtrlApp {
    fn drop(&mut self) {
        self.stop_server();
    }
}

impl eframe::App for RemCtrlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut should_start = false;
        let mut should_stop = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Section 1 — Status panel
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                let status = self.status.lock().unwrap_or_else(|e| e.into_inner());
                match &*status {
                    AppStatus::Idle => {
                        ui.heading("Not running");
                    }
                    AppStatus::Listening { local_ip } => {
                        ui.heading("Waiting for connection...");
                        ui.label(egui::RichText::new(local_ip).small());
                    }
                    AppStatus::Connected { peer_addr } => {
                        ui.heading("Connected");
                        ui.label(egui::RichText::new(peer_addr).small());
                    }
                    AppStatus::Error { message } => {
                        ui.heading(egui::RichText::new(format!("Error: {}", message)).color(egui::Color32::RED));
                    }
                }
                ui.add_space(20.0);
            });

            // Section 2 — Controls
            ui.vertical_centered(|ui| {
                let status = self.status.lock().unwrap_or_else(|e| e.into_inner());
                let is_idle = matches!(*status, AppStatus::Idle | AppStatus::Error { .. });
                drop(status); // Release lock before any action

                if is_idle {
                    if ui.button("Start").clicked() {
                        should_start = true;
                    }
                } else {
                    if ui.button("Stop").clicked() {
                        should_stop = true;
                    }
                }
                ui.add_space(20.0);
            });

            ui.separator();

            // Section 3 — Event log
            ui.heading("Event log");
            let log = self.event_log.lock().unwrap_or_else(|e| e.into_inner());
            if log.is_empty() {
                ui.label(egui::RichText::new("No events yet").color(egui::Color32::GRAY));
            } else {
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for event in log.iter() {
                            ui.label(event);
                        }
                    });
            }
        });

        if should_start {
            self.start_server(ctx.clone());
        }
        if should_stop {
            self.stop_server();
        }
    }
}

fn main() {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 300.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "RemCtrl",
        native_options,
        Box::new(|_cc| Ok(Box::new(RemCtrlApp::new(rt)))),
    )
    .unwrap();
}
