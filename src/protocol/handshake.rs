/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
use protocol::packet::*;
use std::net::TcpStream;

pub struct PktHandshake {
    pub protocol: u32,
    pub address: String,
    pub port: u16,
    pub next_state: u32
}

pub fn rpkt_handshake(stream: &mut TcpStream) -> Result<PktHandshake> {
    let protocol = try!(stream.r_varint());
    let address = try!(stream.r_string());
    let port = try!(stream.r_ushort());
    let next_state = try!(stream.r_varint());
    
    Ok(PktHandshake { protocol: protocol
                    , address: address
                    , port: port
                    , next_state: next_state
                    })
}

pub fn wpkt_handshake(stream: &mut TcpStream, protocol: i32, address: String, port: u16, next_state: i32) -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::VarInt(protocol));
    packet.w_datum(MCDatum::String(address));
    packet.w_datum(MCDatum::UShort(port));
    packet.w_datum(MCDatum::VarInt(next_state));
    packet.w_packet(stream, 0)
}
