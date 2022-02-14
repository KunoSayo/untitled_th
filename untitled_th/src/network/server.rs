use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;

pub enum LogicPacket {}

enum ClientState {}

struct ConnectedClient {
    address: SocketAddr,
    id: usize,
    last_packet_time: std::time::SystemTime,
}

impl ConnectedClient {
    fn new(address: SocketAddr, id: usize) -> Self {
        Self {
            address,
            id,
            last_packet_time: std::time::SystemTime::now(),
        }
    }
}

/// One task running logic and one task to handle network.
/// Two task
pub struct GameServer {
    socket: UdpSocket,
    pub sender: Sender<LogicPacket>,
    receiver: Receiver<LogicPacket>,
    connected: RwLock<HashMap<SocketAddr, ConnectedClient>>,
}


impl GameServer {
    pub async fn new(port: u16) -> tokio::io::Result<Self> {
        log::info!("Binding server port: {}", port);
        let server_socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).await?;
        let (sender, receiver) = tokio::sync::mpsc::channel(1024);

        Ok(Self {
            socket: server_socket,
            sender,
            receiver,
            connected: Default::default(),
        })
    }

    pub async fn loop_network(self: Arc<Self>) -> tokio::io::Result<()> {
        let mut buf = [0; 32 * 1024];
        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((n, addr)) => {
                    if n < crate::network::HEADER_LEN {
                        log::debug!("{:?} sent packet with bytes less than header len", addr);
                        continue;
                    }
                    let mut reader = &buf[..];
                    let (header, mut reader) = reader.split_at(3);
                    if header != crate::network::PACKET_HEADER {
                        log::debug!("{:?} sent packet with wrong header", addr);
                        continue;
                    }

                    let data = Vec::from(&buf[crate::network::HEADER_LEN..n]);

                    let server = self.clone();
                    tokio::spawn(async move {
                        let mut connected = server.connected.read().await;
                        if let Some(client) = connected.get(&addr) {} else {}
                    });
                }
                Err(e) => {
                    log::error!("Receive socket failed for {:?}", e);
                }
            }
        }
        Ok(())
    }
}