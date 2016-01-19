/*
 * Copyright (c) 2016 The TemoMC Devil
 * Distributed under the GNU GPL v3 or higher. For full terms, see the file LICENSE.
 */
use protocol::varint::{ReadVarint, ToVarint};
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use std::string::String;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::result;

pub type Result<T> = result::Result<T, String>;

pub enum MCDatum {
    // Types that only matter for reading will not be
    // implemented, since this is only used for writing,
    // and those kinds of things are handled before
    // interfaces using these are accessed.
    // However, they are still listed merely to have a full list.
    // However, data types not in vanilla that logicially
    // cover more things and are applicable to writing ARE
    // included.
    
    // Optional // Only matters for reading
    // Array(Vec<MCDatum>) // Should be of same type
    // Enum // Only matters for reading
    // ByteArray(Vec<u8>)
    String(String),
    // Chat(),
    // Chunk(),
    // Metadata(),
    // Slot(),
    // NBTTag(),
    // Position(), // 
    // Angle (u8 or i8 [same result]),
    // UUID(u64),
    Bool(bool),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    // VarShort(i16),
    // UInt(u32)
    Int(i32),
    VarInt(i32),
    // Long(u64),
    ULong(u64),
    // VarLong(u64),
    // Float(f32),
    // Double(f64),
}

pub trait MCPacketBuf {
    fn w_datum(&mut self, datum: MCDatum) -> ();
    fn w_packet(&mut self, stream: &mut TcpStream, kind: i32) -> Result<()>;
}

pub trait MCDataStream {
    fn r_string(&mut self) -> Result<String>;
    fn r_bool(&mut self) -> Result<bool>;
    fn r_byte(&mut self) -> Result<i8>;
    fn r_ubyte(&mut self) -> Result<u8>;
    fn r_ushort(&mut self) -> Result<u16>;
    fn r_int(&mut self) -> Result<u32>;
    fn r_varint(&mut self) -> Result<u32>;
    fn r_ulong(&mut self) -> Result<u64>;
}

fn read_n(reader: &mut TcpStream, n: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut buf = &mut [0; 1];
    for _ in 0..n {
        try!(reader.read_exact(buf).or_else(|e| Err(e.to_string())));
        bytes.extend(buf.iter().cloned());
    }
    Ok(bytes)
}

fn read_byte(reader: &mut TcpStream) -> Result<u8> {
    let buf = &mut [0; 1];
    try!(reader.read_exact(buf).or_else(|e| Err(e.to_string())));
    Ok(buf[0])
}

fn write_byte(stream: &mut TcpStream, byte: u8) -> Result<()> {
    let mut buf = Vec::new();
    buf.push(byte);
    stream.write_all(&buf[..]).or_else(|e| Err(e.to_string()))
}

impl MCDataStream for TcpStream {
    fn r_string(&mut self) -> Result<String> {
        let len = try!(self.read_varint());
        let val = try!(read_n(self, len as u64));
        String::from_utf8(val).or_else(|e| Err(e.to_string()))
    }

    fn r_bool(&mut self) -> Result<bool> {
        read_byte(self).map(|x| x != 0)
    }

    fn r_byte(&mut self) -> Result<i8> {
        read_byte(self).map(|x| x as i8)
    }
    
    fn r_ubyte(&mut self) -> Result<u8> {
        read_byte(self)
    }
    
    fn r_ushort(&mut self) -> Result<u16> {
        let buf = try!(read_n(self, 2));
        let mut rdr = Cursor::new(buf);
        rdr.read_u16::<BigEndian>().or_else(|e| Err(e.to_string()))
    }

    fn r_int(&mut self) -> Result<u32> {
        let buf = try!(read_n(self, 4));
        let mut rdr = Cursor::new(buf);
        rdr.read_u32::<BigEndian>().or_else(|e| Err(e.to_string()))
    }
    
    fn r_varint(&mut self) -> Result<u32> {
        self.read_varint()
    }

    fn r_ulong(&mut self) -> Result<u64> {
        let buf = try!(read_n(self, 8));
        let mut rdr = Cursor::new(buf);
        rdr.read_u64::<BigEndian>().or_else(|e| Err(e.to_string()))
    }
}

fn serialize(val: MCDatum) -> Vec<u8> {
    let mut vec = Vec::new();
    match val {
        MCDatum::String(v) => {
            let mut bytes = v.into_bytes();
            let mut length = (bytes.len() as u32).to_varint();
            vec.append(&mut length);
            vec.append(&mut bytes);
        },
        MCDatum::Bool(v) => {
            vec.push(if v { 1 } else { 0 });
        },
        MCDatum::Byte(v) => {
            vec.push(v as u8); // TODO: Fix, this probably doesn't work
        },
        MCDatum::UByte(v) => {
            vec.push(v);
        },
        MCDatum::UShort(v) => {
            vec.write_u16::<BigEndian>(v).unwrap();
        },
        MCDatum::Int(v) => {
            vec.write_i32::<BigEndian>(v).unwrap();
        },
        MCDatum::VarInt(v) => {
            let mut bytes = v.to_varint();
            vec.append(&mut bytes);
        },
        MCDatum::ULong(v) => {
            vec.write_u64::<BigEndian>(v).unwrap();
        },
        _ => {
            unimplemented!();
        }
    }
    vec
}

impl MCPacketBuf for Vec<u8> {
    fn w_datum(&mut self, datum: MCDatum) -> () {
        let mut result = serialize(datum);
        self.append(&mut result);
    }
    
    fn w_packet(&mut self, stream: &mut TcpStream, kind: i32) -> Result<()> {
        let size = (self.len() as u32 + 1).to_varint();
        let kind = (kind as u32).to_varint();
        try!(stream.write_all(&size[..]).or_else(|e| Err(e.to_string())));
        try!(stream.write_all(&kind[..]).or_else(|e| Err(e.to_string())));
        stream.write_all(&self[..]).or_else(|e| Err(e.to_string()))
    }
}


pub fn read_packet(stream: &mut TcpStream) -> Result<u32> {
    try!(stream.r_varint());
    stream.r_varint()
}
