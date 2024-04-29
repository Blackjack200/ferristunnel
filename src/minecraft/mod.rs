use std::collections::HashMap;

use std::io::{Read, Write};
use std::io::Result;

use bstream::{BinaryStream, ReaderExt, WriterExt};

use crate::minecraft::packets::*;

#[allow(dead_code)]
pub enum ProtocolId {
    V1_20_80 = 671,
    V1_20_70 = 662,
    V1_20_60 = 649,
    V1_20_50 = 630,
    V1_20_40 = 622,
    V1_20_30 = 618,
    V1_20_10 = 594,
    V1_20_0 = 589,
}

pub mod packets;

pub trait Protocol {
    fn id() -> i32;
    fn version() -> &'static str;
    fn pool() -> HashMap<i32, PacketKind>;
    fn read_packet(pool: &HashMap<i32, PacketKind>, r: &mut impl Read) -> Result<PacketKind>;
    fn write_packet<T: Packet + BinaryStream>(w: &mut impl Write, pk: &T) -> Result<()>;
}

pub struct DefaultProtocol {}

impl Protocol for DefaultProtocol {
    fn id() -> i32 {
        ProtocolId::V1_20_80 as i32
    }

    fn version() -> &'static str {
        "v1.20.80"
    }

    fn pool() -> HashMap<i32, PacketKind> {
        HashMap::from([
            (
                RequestNetworkSettingsPacket::id(),
                RequestNetworkSettingsPacket::kind(),
            ),
            (NetworkSettingsPacket::id(), NetworkSettingsPacket::kind()),
            (LoginPacket::id(), LoginPacket::kind()),
        ])
    }

    fn read_packet(pool: &HashMap<i32, PacketKind>, r: &mut impl Read) -> Result<PacketKind> {
        let header = r.read_vi32()?;
        let pid = header & 0x3ff;
        let kind = pool.get(&pid).unwrap();
        decode_kind(r, kind)
    }

    fn write_packet<T: Packet + BinaryStream>(w: &mut impl Write, pk: &T) -> Result<()> {
        w.write_vi32(T::id())?;
        pk.write(w)
    }
}

pub trait Packet {
    fn compressible() -> bool;
    fn id() -> i32;
    fn kind() -> PacketKind;
}

#[inline]
fn decode<T: Packet + Clone + BinaryStream>(pk: &T, r: &mut impl Read) -> Result<T> {
    let mut cpk = pk.clone();
    cpk.read(r)?;
    Ok(cpk)
}
