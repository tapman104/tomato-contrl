# RemCtrl

RemCtrl is a desktop remote-control application server built in Rust. It allows a remote client (like a mobile app) to send control events over the local network to inject mouse movements and keyboard strokes seamlessly.

## Features

* **Desktop UI (egui)**: A clean, single-window desktop application interface built with `egui` and `eframe`. It displays the current server state, peer address, and your local IP so remote clients know exactly where to connect.
* **OS-Level Input Injection**: Leverages the `enigo` crate to seamlessly inject inputs directly into the host operating system.
  * **Mouse Support**: Smooth relative mouse cursor movements, left/right/middle clicks, and scroll wheel events.
  * **Keyboard Support**: Full alphanumeric typing, modifier keys (Ctrl, Alt, Shift, etc.), and special key injection.
* **Zero-Configuration Discovery**: Automatically advertises the server on the local network using mDNS (Multicast DNS). Compatible clients can automatically discover and connect to the server without manually entering IP addresses.
* **Asynchronous Networking**: Built on top of `tokio` for efficient, non-blocking TCP communication, ensuring low-latency event processing.
* **Real-time Event Logging**: Features a scrollable event log built directly into the UI that parses and displays the latest remote inputs received in real-time.
* **Cross-Platform Potential**: While currently optimized for Windows OS-level injection, the underlying Rust stack (tokio, egui, serde) provides a solid foundation for cross-platform compatibility.

## Tech Stack
* **Language**: Rust
* **UI**: `egui`, `eframe`
* **Async Runtime**: `tokio`
* **Networking**: `mdns-sd` (Discovery), Standard TCP
* **Input Injection**: `enigo`
* **Serialization**: `serde`, `serde_json`

## Getting Started

To run the desktop server, make sure you have the Rust toolchain installed, and run:

```bash
cargo run --bin remctrl
```

Once running, simply click "Start" from the UI. The application will begin listening for connections and advertising itself on your local network.
