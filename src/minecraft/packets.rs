use std::io;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::minecraft::{decode, Packet};
use crate::minecraft::packets::PacketKind::RequestNetworkSettings;

macro_rules! def_pk {
    ($id:tt,$c:tt,$kind:tt) => {
        fn compressible() -> bool { return $c; }
        fn id() -> i32 { return $id; }
        fn kind() -> PacketKind { return $kind(Default::default()); }
    };
}

pub(crate) enum PacketKind {
    RequestNetworkSettings(RequestNetworkSettingsPacket)
}

pub fn decode_kind(r: &mut impl Read, kind: &PacketKind) -> io::Result<PacketKind> {
    Ok(match kind {
        RequestNetworkSettings(pk) => RequestNetworkSettings(decode(pk, r)?)
    })
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RequestNetworkSettingsPacket {
    pub(crate) protocol: i32,
}

impl Packet for RequestNetworkSettingsPacket {
    def_pk!(0xc1,false,RequestNetworkSettings);
    fn from(&mut self, out: &mut impl Read) -> io::Result<()> {
        self.protocol = out.read_i32::<BigEndian>()?;
        return Ok(());
    }

    fn write(&self, out: &mut impl io::Write) -> io::Result<()> {
        out.write_i32::<BigEndian>(self.protocol)
    }
}