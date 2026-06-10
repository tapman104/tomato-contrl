//! Transport definitions for remctrl.
pub mod tcp;

use async_trait::async_trait;
use crate::protocol::ControlEvent;

/// Transport trait representing a connection to a remote client.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Receive the next ControlEvent from the remote client.
    /// Blocks until an event arrives or the connection closes.
    async fn recv(&mut self) -> anyhow::Result<ControlEvent>;

    /// Gracefully close the connection.
    async fn close(&mut self);

    /// Human-readable name of the transport mode.
    fn mode_name(&self) -> &'static str;
}

/// Represents the current connection state of a transport.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Waiting for a connection.
    Listening,
    /// Connected to a remote client.
    Connected { 
        /// Remote IP:port as a string.
        peer_addr: String 
    },
    /// Connection has been closed or dropped.
    Disconnected,
}
