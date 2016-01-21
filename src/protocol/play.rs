/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
use protocol::packet::*;
use std::net::TcpStream;

pub fn wpkt_join_game(stream: &mut TcpStream, eid: i32, gamemode: u8,
                      dimension: u8, difficulty: u8, max_players: u8,
                      level_type: String, reduced_debug_info: bool)
                      -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::Int(eid));
    packet.w_datum(MCDatum::UByte(gamemode));
    packet.w_datum(MCDatum::UByte(dimension));
    packet.w_datum(MCDatum::UByte(difficulty));
    packet.w_datum(MCDatum::UByte(max_players));
    packet.w_datum(MCDatum::String(level_type));
    packet.w_datum(MCDatum::Bool(reduced_debug_info));
    packet.w_packet(stream, 1)
}

// rpkt_join_game

pub fn wpkt_spawn_position(stream: &mut TcpStream, x: i32, y: i32, z: i32)
                       -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::Position(x, y, z));
    packet.w_packet(stream, 5)
}

// rpkt_spawn_position

pub fn wpkt_player_pal(stream: &mut TcpStream,
                   x: f64, y: f64, z: f64,
                   yaw: f32, pitch: f32,
                   flags: u8) -> Result<()> {
    let mut packet = Vec::new();
    packet.w_datum(MCDatum::Double(x));
    packet.w_datum(MCDatum::Double(y));
    packet.w_datum(MCDatum::Double(z));
    packet.w_datum(MCDatum::Float(yaw));
    packet.w_datum(MCDatum::Float(pitch));
    packet.w_datum(MCDatum::UByte(flags));
    packet.w_packet(stream, 8)
}

// rpkt_player_pal
