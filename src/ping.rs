use std::io::{Read, Result, Write};
use std::net::TcpStream;
use crate::packet::Packet;

pub fn handle_server_list_ping(stream: &mut TcpStream) -> Result<()> {
    // Emptying the read buffer
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer)?;
    // Sending server infos
    let payload = json::parse(r#"
        {
            "description": {
                "text": "Minecraft Server by Arnaud PERALTA with Rust"
            },
            "players": {
                "max": 10,
                "online": 100000
            },
            "version": {
                "name": "1.18.2",
                "protocol": 758
            }
        }
    "#).unwrap().dump();

    let mut packet = Packet {
        packet_id: 0x00,
        payload: Vec::new()
    };
    packet.write_string(String::from(payload))?;
    packet.send(stream)?;

    packet.payload = Vec::new();
    stream.read(&mut buffer)?; // PING
    stream.write(&mut buffer)?; // PONG
    stream.flush()
}
