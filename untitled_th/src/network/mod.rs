//! Network module for game
//! For general packet format:
//! Contain PACKET_HEADER VERSION(be u16) TYPE(be u16)  
//! Following is packet data.

pub const PACKET_HEADER: [u8; 3] = *b"uth";
pub const VERSION: u16 = 0;
pub const HEADER_LEN: usize = PACKET_HEADER.len() + std::mem::size_of::<u16>() * 2;

pub mod server;

pub mod packets {
    /// and following u8 as ping increasing index
    pub const PING: u16 = 0;
    /// and following u8 as pong index
    pub const PONG: u16 = 1;
    /// For client to server the following u8 as index(zero for first) and left all is username if first.
    /// For server to client the following u8 as token to send again and remaining for message if present.
    /// In all, client need to send twice connect packet and server send once packet to build the connection.
    pub const CONNECT: u16 = 2;
}
