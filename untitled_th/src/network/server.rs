use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use byteorder::{BE, ReadBytesExt};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;

use super::packets;

pub enum LogicPacket {}

enum ClientState {
    Connecting(u8),
    Idle,
    InRoom,
    Gaming,
    Unknown,
}

struct ConnectedClient {
    address: SocketAddr,
    state: ClientState,
    id: usize,
    last_packet_time: std::time::SystemTime,
}

impl ConnectedClient {
    fn new(address: SocketAddr, id: usize, token: u8) -> Self {
        Self {
            address,
            state: ClientState::Connecting(token),
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

    async fn run_clean_task(self: Arc<Self>) {
        let sleep_dur = Duration::from_secs(30);
        loop {
            tokio::time::sleep(sleep_dur).await;
        }
    }

    /// loop network and should be tokio runtime
    pub async fn run_network(self: Arc<Self>) -> tokio::io::Result<()> {
        let mut buf = [0; 32 * 1024];
        // the connection manager.
        tokio::spawn(self.clone().run_clean_task());
        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((n, addr)) => {
                    let _now = std::time::SystemTime::now();
                    let _now_ins = std::time::Instant::now();

                    if n < crate::network::HEADER_LEN {
                        log::debug!("{:?} sent packet with bytes less than header len", addr);
                        continue;
                    }
                    let reader = &buf[..];
                    let (header, mut reader) = reader.split_at(3);
                    if header != crate::network::PACKET_HEADER {
                        log::debug!("{:?} sent packet with wrong header", addr);
                        continue;
                    }
                    // we checked the length
                    let version = reader.read_u16::<BE>().unwrap();
                    let packet_type = reader.read_u16::<BE>().unwrap();

                    let data = Vec::from(&buf[crate::network::HEADER_LEN..n]);
                    let server = self.clone();
                    tokio::spawn(async move {
                        let connected = server.connected.read().await;
                        if let Some(client) = connected.get(&addr) {} else {
                            match packet_type {
                                packets::CONNECT => {}
                                _ => {
                                    log::debug!("{:?} sent packet but not connected", addr);
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    log::error!("Receive socket failed for {:?}", e);
                }
            }
        }
    }
}
