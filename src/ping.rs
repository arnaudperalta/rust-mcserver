use std::io::{Read, Write, Result};
use std::net::TcpStream;
use mc_varint::{VarInt, VarIntWrite};

pub fn handle_server_list_ping(mut stream: TcpStream) -> Result<()> {
    let payload = json::parse(r#"
        {
            "description": {
                "text": "Minecraft Server by Arnaud PERALTA with Rust"
            },
            "players": {
                "max": 10,
                "online": 1337
            },
            "version": {
                "name": "1.18.2",
                "protocol": 758
            }
        }
    "#).unwrap().dump();
    // Yes add 3 because why not ?
    stream.write_var_int(VarInt::from(payload.len() as i32 + 3))?;
    stream.write(&mut [0x00])?;
    stream.write_var_int(VarInt::from(payload.len() as i32))?;
    // stream.write(&payload_length)?;
    stream.write(&payload.as_bytes())?;
    stream.flush()?;
    // Wait for client ping
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    stream.write(&mut buffer)?;
    stream.flush()
}
