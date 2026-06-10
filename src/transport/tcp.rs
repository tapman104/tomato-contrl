//! TCP Transport implementation.
use super::Transport;
use crate::protocol::ControlEvent;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

/// Default port for the TCP transport.
pub const DEFAULT_PORT: u16 = 9847;

/// A transport implementation that uses a TCP stream.
pub struct TcpTransport {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::net::tcp::OwnedWriteHalf,
    peer_addr: String,
}

impl TcpTransport {
    /// Bind to 0.0.0.0:9847 and wait for exactly one client 
    /// to connect. Returns when a client has connected.
    /// This call blocks (async) until the first connection.
    pub async fn accept() -> Result<Self> {
        let listener = TcpListener::bind(("0.0.0.0", DEFAULT_PORT)).await?;
        let (stream, addr) = listener.accept().await?;
        let peer_addr = addr.to_string();
        
        let (read_half, write_half) = stream.into_split();
        let reader = BufReader::new(read_half);
        
        Ok(Self {
            reader,
            writer: write_half,
            peer_addr,
        })
    }

    /// Returns the remote IP:port as a string.
    pub fn peer_addr(&self) -> &str {
        &self.peer_addr
    }
}

#[async_trait]
impl Transport for TcpTransport {
    async fn recv(&mut self) -> Result<ControlEvent> {
        let mut line = String::new();
        let bytes_read = self.reader.read_line(&mut line).await?;
        
        if bytes_read == 0 {
            return Err(anyhow!("connection closed by peer"));
        }
        
        let event: ControlEvent = serde_json::from_str(&line)?;
        Ok(event)
    }

    async fn close(&mut self) {
        let _ = self.writer.shutdown().await;
    }

    fn mode_name(&self) -> &'static str {
        "TCP/Local"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_send_and_receive() {
        let server_task = tokio::spawn(async {
            TcpTransport::accept().await.unwrap()
        });

        // Give the server a moment to start listening
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let mut stream = TcpStream::connect(("127.0.0.1", DEFAULT_PORT)).await.unwrap();
        stream.write_all(b"{\"type\":\"MouseMove\",\"dx\":10.0,\"dy\":20.0}\n").await.unwrap();
        
        let mut transport = server_task.await.unwrap();
        let event = transport.recv().await.unwrap();
        
        assert_eq!(event, ControlEvent::MouseMove { dx: 10.0, dy: 20.0 });
    }
}
