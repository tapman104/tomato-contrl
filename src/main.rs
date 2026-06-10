//! Remctrl binary entry point.
use remctrl::server::run_server;
use remctrl::transport::ConnectionState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::spawn(async {
        let _ = tokio::signal::ctrl_c().await;
        std::process::exit(0);
    });

    println!("RemCtrl listening on port 9847...");

    run_server(
        |state: ConnectionState| {
            println!("[STATE] {:?}", state);
        },
        |event| {
            let event_json = serde_json::to_string(event).unwrap_or_else(|_| "{}".to_string());
            println!("[EVENT] {}", event_json);
        },
    )
    .await?;

    println!("Disconnected. Exiting.");
    Ok(())
}
