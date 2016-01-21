/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
use protocol::packet::*;
use std::net::TcpStream;

pub fn wpkt_login_start(stream: &mut TcpStream) -> Result<()> {
    let mut packet = Vec::new();
    packet.w_packet(stream, 1)
}

pub fn rpkt_login_start(stream: &mut TcpStream) -> Result<()> {
    Ok(())
}

// Encryption request

// Encryption response

pub fn wpkt_login_success(stream: &mut TcpStream,
                          uuid: String,
                          username: String)
                          -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::String(uuid));
    packet.w_datum(MCDatum::String(username));
    packet.w_packet(stream, 2)
}

// rpkt_login_success

// Set compression
