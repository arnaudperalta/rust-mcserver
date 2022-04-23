use std::net::TcpStream;
use std::io::{Read, Result};
use std::str::from_utf8;
use varint::VarintRead;

pub fn read_string(stream: &mut TcpStream) -> Result<String> {
    let mut buffer = vec![];
    let str_length = stream.read_unsigned_varint_32().unwrap();
    let mut str_stream = stream.take(str_length as u64);
    str_stream.read_to_end(&mut buffer)?;
    Ok(String::from(from_utf8(&buffer).unwrap()))
}

pub fn read_varint(stream: &mut TcpStream) -> Result<u32> {
    stream.read_unsigned_varint_32()
}

pub fn read_ushort(stream: &mut TcpStream) -> Result<u16> {
    let mut buffer = [0u8, 2];
    stream.read(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

pub fn write_string(stream: &mut TcpStream, value: String) -> Result<()> {
    Ok(())
}

pub fn write_varint(stream: &mut TcpStream, value: u32) -> Result<()> {
    Ok(())
}
