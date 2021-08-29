use crate::server::server::PacketHandler;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;

#[derive(Clone)]
pub struct MapServer;

impl PacketHandler for MapServer {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        println!("Map");
        Result::Ok("res".to_string())
    }
}