use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::io::Result;

use bstream::{ReaderExt, WriterExt};

use crate::minecraft::packets::{decode_kind, PacketKind, RequestNetworkSettingsPacket};

pub(crate) mod packets;

pub trait Decode {
    fn decode(&self, in_: &impl io::Read) -> Result<PacketKind>;
}

pub trait Packet {
    fn compressible() -> bool;
    fn id() -> i32;
    fn kind() -> PacketKind;
    fn from(&mut self, out: &mut impl io::Read) -> Result<()>;
    fn write(&self, out: &mut impl io::Write) -> Result<()>;
}

fn decode<T: Packet + Clone>(pk: &T, r: &mut impl Read) -> Result<T> {
    let mut cpk = pk.clone();
    cpk.from(r)?;
    Ok(cpk)
}

pub(crate) fn read_packet(r: &mut impl Read) -> Result<PacketKind> {
    let header = r.read_vi32()?;
    let pid = header & 0x3ff;
    let map: HashMap<i32, PacketKind> = HashMap::from([
        (RequestNetworkSettingsPacket::id(), RequestNetworkSettingsPacket::kind())
    ]);
    let kind = map.get(&pid).unwrap();
    decode_kind(r, kind)
}

pub(crate) fn write_packet<T: Packet>(w: &mut impl Write, pk: &T) -> io::Result<()> {
    w.write_vi32(T::id())?;
    pk.write(w)
}
