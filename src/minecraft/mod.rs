use std::io;
use std::io::{Read, Write};
use std::io::Result;

use bstream::{BinaryStream, ReaderExt, WriterExt};

use crate::minecraft::packets::{
    decode_kind, packet_pool, PacketKind,
};

pub mod packets;

pub trait Packet {
    fn compressible() -> bool;
    fn id() -> i32;
    fn kind() -> PacketKind;
}

fn decode<T: Packet + Clone + BinaryStream>(pk: &T, r: &mut impl Read) -> Result<T> {
    let mut cpk = pk.clone();
    cpk.read(r)?;
    Ok(cpk)
}

pub fn read_packet(r: &mut impl Read) -> Result<PacketKind> {
    let header = r.read_vi32()?;
    let pid = header & 0x3ff;
    let map = packet_pool();
    let kind = map.get(&pid).unwrap();
    decode_kind(r, kind)
}

pub fn write_packet<T: Packet + BinaryStream>(w: &mut impl Write, pk: &T) -> io::Result<()> {
    w.write_vi32(T::id())?;
    pk.write(w)
}
