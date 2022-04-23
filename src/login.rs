use std::io::{Read, Result, Write};
use std::net::TcpStream;
use mc_varint::VarInt;
use uuid::Uuid;
use crate::packet::Packet;
use crate::protocol;

pub fn client_login(stream: &mut TcpStream) -> Result <()> {
    // Login Start
    let client_username = protocol::read_string(stream)?;
    println!("Client name:  {}", client_username);

    // // Set compression threshold to max
    let mut compression_packet = Packet {
        packet_id: 0x03,
        payload: Vec::new()
    };
    compression_packet.write_varint(VarInt::from(65535))?;
    compression_packet.send(stream)?;

    // Login Success
    let mut login_packet = Packet {
        packet_id: 0x02,
        payload: Vec::new()
    };
    login_packet.write_uuid(Uuid::new_v4())?;
    login_packet.write_string(client_username)?;
    login_packet.send(stream)
    // protocol::write_uuid(stream, Uuid::new_v4())?;
    // protocol::write_string(stream, client_username)?;
    // stream.flush()
}