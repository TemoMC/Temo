/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
use protocol::packet::*;
use std::net::TcpStream;

// Pings and pongs could to combined into a pingpong since
// they're the same thing, and I could combine read ping and
// write pong into a single automatic function, but I decided
// not to assume anything and keep the API uniform.

pub fn wpkt_ping(stream: &mut TcpStream, val: u64) -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::ULong(val));
    packet.w_packet(stream, 1)
}

pub fn rpkt_ping(stream: &mut TcpStream) -> Result<u64> {
    stream.r_ulong()
}

pub fn wpkt_pong(stream: &mut TcpStream, val: u64) -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::ULong(val));
    packet.w_packet(stream, 1)
}

pub fn rpkt_pong(stream: &mut TcpStream) -> Result<u64> {
    stream.r_ulong() // You bet. Whoops! Connection over, I guess not.
}

pub fn wpkt_request(stream: &mut TcpStream) -> Result<()> {
    // I really wish these status packets weren't so difficult :/
    let mut packet = Vec::new();
    packet.w_packet(stream, 0)
}

// Does nothing, but included for completion
pub fn rpkt_request(stream: &mut TcpStream) -> Result<()> {
    Ok(())
}

pub fn wpkt_response(stream: &mut TcpStream, name: String, protocol: u8, maxplayers: u16, curplayers: u16, description: String) -> Result<()> {
    let mut packet = Vec::new();
    // TODO: Do this the right way
    // TODO: Support sample players
    packet.w_datum(MCDatum::String(format!("{{ \"version\": {{ \"name\": \"{}\", \"protocol\": {} }}, \"players\": {{ \"max\": {}, \"online\": {} }}, \"description\": {{ \"text\": \"{}\" }} }}", name, protocol, maxplayers, curplayers, description)));
    packet.w_packet(stream, 0)
}

// rpkt_response
