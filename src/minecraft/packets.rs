use std::collections::HashMap;
use std::io;
use std::io::Read;

use bstream::Vu32LenByteSlice;
use bstream_macro::BStream;

use crate::minecraft::{decode, Packet};
use crate::minecraft::packets::PacketKind::*;

macro_rules! register_pk {
    ($name:tt,$id:tt,$c:tt,$kind:tt) => {
        impl Packet for $name {
            fn compressible() -> bool {
                return $c;
            }
            fn id() -> i32 {
                return $id;
            }
            fn kind() -> PacketKind {
                return $kind(Default::default());
            }
        }
    };
}

pub fn packet_pool() -> HashMap<i32, PacketKind> {
    HashMap::from([
        (
            RequestNetworkSettingsPacket::id(),
            RequestNetworkSettingsPacket::kind(),
        ),
        (NetworkSettingsPacket::id(), NetworkSettingsPacket::kind()),
    ])
}

pub enum PacketKind {
    RequestNetworkSettings(RequestNetworkSettingsPacket),
    NetworkSettings(NetworkSettingsPacket),
    Login(LoginPacket),
}

pub fn decode_kind(r: &mut impl Read, kind: &PacketKind) -> io::Result<PacketKind> {
    Ok(match kind {
        RequestNetworkSettings(pk) => RequestNetworkSettings(decode(pk, r)?),
        NetworkSettings(pk) => NetworkSettings(decode(pk, r)?),
        Login(pk) => Login(decode(pk, r)?),
    })
}

/// RequestNetworkSettingsPacket is sent by the client to request network settings, such as compression, from the server.
#[derive(Debug, Clone, Default, BStream)]
pub struct RequestNetworkSettingsPacket {
    /// client_protocol is the protocol version of the player. The player is disconnected if the protocol is
    /// incompatible with the protocol of the server.
    #[Varint]
    pub client_protocol: i32,
}

register_pk!(
    RequestNetworkSettingsPacket,
    0xc1,
    false,
    RequestNetworkSettings
);

/// NetworkSettingsPacket is sent by the server to update a variety of network settings. These settings modify the
/// way packets are sent over the network stack.
#[derive(Debug, Clone, Default, BStream)]
pub struct NetworkSettingsPacket {
    /// compression_threshold is the minimum size of a packet that is compressed when sent. If the size of a
    /// packet is under this value, it is not compressed.
    /// When set to 0, all packets will be left uncompressed.
    pub compression_threshold: u16,
    /// compression_algorithm is the algorithm that is used to compress packets.
    pub compression_algorithm: u16,

    /// client_throttle regulates whether the client should throttle players when exceeding of the threshold. Players
    /// outside threshold will not be ticked, improving performance on low-end devices.
    pub client_throttle: bool,
    /// client_throttle_threshold is the threshold for client throttling. If the number of players exceeds this value, the
    /// client will throttle players.
    pub client_throttle_threshold: u8,
    /// client_throttle_scalar is the scalar for client throttling. The scalar is the amount of players that are ticked
    /// when throttling is enabled.
    pub client_throttle_scalar: f32,
}

register_pk!(NetworkSettingsPacket, 0x8f, false, NetworkSettings);

/// LoginPacket is sent when the client initially tries to join the server. It is the first packet sent and contains
/// information specific to the player.
#[derive(Debug, Clone, Default, BStream)]
pub struct LoginPacket {
    /// client_protocol is the protocol version of the player. The player is disconnected if the protocol is incompatible
    /// with the protocol of the server. It has been superseded by the protocol version sent in the
    /// RequestNetworkSettings packet, so this should no longer be used by the server.
    pub client_protocol: i32,
    /// connection_request is a string containing information about the player and JWTs that may be used to
    /// verify if the player is connected to XBOX Live. The connection request also contains the necessary
    /// client public key to initiate encryption.
    pub connection_request: Vu32LenByteSlice,
}

register_pk!(LoginPacket, 0x01, true, Login);
