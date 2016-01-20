/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
extern crate byteorder;

// BIG TODO: Encryption, compression

use std::net::{TcpListener, TcpStream};
use std::thread;

mod protocol;

use protocol::packet::{read_packet};
use protocol::status::*;
use protocol::handshake::*;
use protocol::login::*;

fn handle_client(stream: TcpStream) {
    // Note: Panicking is okay in this function, since there's nothing we can
    // clean up. However, in the play function, there may be clean up and
    // saving to do, so there will be proper error handling there.
    let mut stream = stream;
    let ip = stream.peer_addr().unwrap();
    println!("Recieved connection on {} from {}",
             stream.local_addr().unwrap(),
             ip);

    let packet_type = read_packet(&mut stream).unwrap();
    
    let mut next_state = 0;

    // Note: Whenever an invalid packet is detected, we just
    // exit, since it probably means the connection is dead
    // forever (since we won't read what comes next). We could
    // probably work around this using the packet length, but
    // I figure an indev client sending bad packets will want
    // to die, and older clients just won't send bad packets.
    // TCP assures there is no corruption in transmission, too.
    
    match packet_type {
        // Handshake packet
        0 => {
            // This line now seems way too long,
            // but I'm not sure how to properly break it
            let packet = rpkt_handshake(&mut stream).unwrap();
            println!("Handshake from {}: \
                      protocol {}, \
                      address {}, \
                      port {}, \
                      next state {}",
                     ip,
                     packet.protocol,
                     packet.address,
                     packet.port,
                     packet.next_state);
            next_state = packet.next_state;
        },
        _ => {
            println!("Invalid packet for handshake of type {} from {}!",
                     packet_type,
                     ip);
            return;
        }
    }

    match next_state {
        1 => {
            let packet_type = read_packet(&mut stream).unwrap();
            if packet_type != 0 {
                println!("Bad packet type for status from {}!", ip);
                return;
            }
            
            rpkt_request(&mut stream).unwrap();
            println!("Request for status from {}", ip);
            // Oh boy, my formatting is already inconsistent. I need to
            // figure out a style for breaking lines later.
            wpkt_response(&mut stream, "1.8.9".to_string(), 47, 100, 0,
                          "Hello, world!".to_string()).unwrap();

            let packet_type = read_packet(&mut stream).unwrap();
            
            if packet_type != 1 {
                println!("Bad packet type for status from {}!", ip);
                return;
            }
            let v = rpkt_ping(&mut stream).unwrap();
            println!("Status ping from {}", ip);
            wpkt_pong(&mut stream, v).unwrap();
        }
        2 => {
            let packet_type = read_packet(&mut stream).unwrap();
            if packet_type != 0 {
                println!("Expected login start from {}, got {}!",
                        ip, packet_type);
                return;
            }

            // Encryption

            // Authentication
            
            wpkt_login_success(&mut stream,
                               // The actual UUID of the user "Player"
                               "bd346dd5-ac1c-427d-87e8-73bdd4bf3e13"
                               .to_string(),
                               "Player".to_string());

            // Compression
        }
        n => {
            println!("Invalid state {} from {}!", n, ip);
            return;
        }
    }

    println!("{} lost connection: Fell out of loop!", ip);
    drop(stream);
}

fn main() {
    // TODO: Configuration for bound host and port
    let listener = TcpListener::bind("0.0.0.0:35565")
        .expect("Could not bind to IP and port!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::Builder::new()
                    .name("Handler thread".to_string())
                    .spawn(move|| {
                        handle_client(stream);
                    });
            },
            Err(_) => { }
        }
    }
    drop(listener);
}
