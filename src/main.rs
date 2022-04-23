use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};
use mc_varint::{VarIntRead};
use crate::ping::handle_server_list_ping;

mod packet_id;
mod ping;
mod login;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let res = handle_connection(stream);
        if let Err(err) = res {
            println!("{:?}", err);
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let packet_id = stream.read_var_int().unwrap();
    let _ = stream.read(&mut buffer);
    match packet_id.into() {
        0x10 => handle_server_list_ping(stream)?,
        _ => ()
    }
    Ok(())
}
