//! Network module for game
//! For general packet format:
//! Contain PACKET_HEADER VERSION(be u16) TYPE(be u16)


pub const PACKET_HEADER: [u8; 3] = *b"uth";
pub const VERSION: u16 = 0;
pub const HEADER_LEN: usize = PACKET_HEADER.len() + std::mem::size_of::<u16>() * 2;

pub mod server;

#[repr(u16)]
pub enum PacketType {
    /// and following u8 as ping increasing index
    Ping = 0,
    /// and following u8 as pong index
    Pong = 1,
    /// For client to server the following u8 as index(zero for first) and left all is username if first.
    /// For server to client the following u8 as bool and remaining for message.
    Connect = 2,
}