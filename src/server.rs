//! High-level server loop.
use crate::discovery;
use crate::input;
use crate::protocol::ControlEvent;
use crate::transport::tcp::TcpTransport;
use crate::transport::{ConnectionState, Transport};

/// Run the remctrl server loop.
///
/// This function:
///   1. Starts mDNS advertisement via discovery::advertise()
///   2. Waits for one TCP connection via TcpTransport::accept()
///   3. Loops: recv() each ControlEvent → input::inject()
///   4. On disconnect: stops injecting, returns Ok(())
///   5. The mDNS advertisement is stopped when the function returns
///
/// The on_state_change callback is called at each transition.
/// The on_event callback is called after each successfully received and injected event.
/// Errors from inject() are logged but do not stop the loop.
/// Errors from recv() (including disconnect) stop the loop.
pub async fn run_server<F, G>(
    on_state_change: F,
    on_event: G,
) -> anyhow::Result<()>
where
    F: Fn(ConnectionState) + Send + 'static,
    G: Fn(&ControlEvent) + Send + 'static,
{
    // Start mDNS advertisement
    let _mdns_handle = discovery::advertise()?;

    // Wait for connection
    on_state_change(ConnectionState::Listening);
    let mut transport = TcpTransport::accept().await?;
    let peer_addr = transport.peer_addr().to_string();
    on_state_change(ConnectionState::Connected { peer_addr });

    // Loop
    loop {
        match transport.recv().await {
            Ok(event) => {
                if let Err(e) = input::inject(&event) {
                    eprintln!("Injection error: {}", e);
                }
                on_event(&event);
            }
            Err(_) => {
                // Connection closed or recv error
                break;
            }
        }
    }

    on_state_change(ConnectionState::Disconnected);
    Ok(())
}
