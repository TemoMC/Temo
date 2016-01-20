/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
use std::io::{Read};
use std::net::{TcpStream};
use protocol::packet::{Result};

pub trait ReadVarint {
    fn read_varint(&mut self) -> Result<u32>;
}

pub trait ToVarint {
    fn to_varint(&self) -> Vec<u8>;
}

impl ReadVarint for TcpStream {
    fn read_varint(&mut self) -> Result<u32> {
        let mut result = 0;
        // The max varint size is 4 bytes, so limit it there
        for i in 0..4 {
            let mut buf = &mut [0; 1];
            try!(self.read_exact(buf).or_else(|e| Err(e.to_string())));
            let byte = buf[0];
            // Get the numerical part of the byte and shift it into our complete integer
            result |= (((byte & 0b0111111) as u8) as u32) << (i * 7);
            // The flag bit isn't on, so just quit
            if (byte >> 7) == 0 {
                break;
            } else if i == 3 {
                return Err("Varint too long!".to_string());
            }
        }
        Ok(result)
    }
}

impl ToVarint for u32 {
    fn to_varint(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut working = self.clone();
        loop {
            // Get the 7 bits in the current byte, and zero
            // the flag bit
            let byte = (working & 0x7F) as u8;
            working >>= 7;
            if working == 0 {
                // No, so just push the final bits and exit
                result.push(byte);
                break;
            } else {
                // There are more, so tell the client that too
                result.push(byte | 0x80);
            }
        }
        result
    }
}

// NOTE: as u8 probably doesn't work
impl ToVarint for i32 {
    fn to_varint(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut working = self.clone();
        loop {
            let byte = (working & 0x7F) as u8;
            working >>= 7;
            if working == 0 {
                result.push(byte);
                break;
            } else {
                result.push(byte | 0x80);
            }
        }
        result
    }
}
