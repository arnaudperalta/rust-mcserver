use std::io::{Result};
use std::net::{TcpListener, TcpStream};

mod packet_id;
mod ping;
mod login;
mod protocol;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let res = handle_connection(&mut stream);
        if let Err(err) = res {
            println!("{:?}", err);
        }
    }
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let _ = protocol::read_varint(stream);
    let packet_id = protocol::read_varint(stream).unwrap();
    println!("Packet ID: {}", packet_id);
    match packet_id {
        0x00 => initializing_connection(stream)?,
        _ => ()
    }
    println!("------");
    Ok(())
}

fn initializing_connection(stream: &mut TcpStream) -> Result<()> {
    // Reading Next State value to determine if this is a login or ping attempt
    let protocol_version = protocol::read_varint(stream).unwrap();
    println!("Protocol Version: {}", protocol_version);
    // Server address
    let server_addr = protocol::read_string(stream).unwrap();
    println!("Server Address: {}", server_addr);
    let server_port = protocol::read_ushort(stream).unwrap();
    println!("Server Port: {}", server_port);
    let next_state = protocol::read_varint(stream).unwrap();
    println!("Next State: {}", next_state);
    match next_state {
        2 => login::client_login()?,
        _ => ping::handle_server_list_ping(stream)?
    }
    Ok(())
}
