use std::io::{Read, Write, Result};
use std::net::TcpStream;
use varint::VarintWrite;

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
    // Yes add 3 because why not ?
    stream.write_unsigned_varint_32((payload.len() + 3) as u32)?;
    stream.write(&mut [0x00])?;
    stream.write_unsigned_varint_32(payload.len() as u32)?;
    stream.write(&payload.as_bytes())?;
    stream.flush()?;
    // Wait for client ping
    stream.read(&mut buffer)?;
    stream.write(&mut buffer)?;
    stream.flush()
}
