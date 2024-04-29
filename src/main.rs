use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;

use crate::minecraft::*;
use crate::minecraft::packets::{CompressionAlgorithm, NetworkSettingsPacket};
use crate::minecraft::packets::PacketKind::*;

mod minecraft;

fn main() {
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open("wow.txt")
        .unwrap();

    let pk = NetworkSettingsPacket {
        compression_threshold: 0,
        compression_algorithm: CompressionAlgorithm::Zlib,
        client_throttle: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    };
    println!("{:?}", &pk);

    DefaultProtocol::write_packet(&mut f, &pk).unwrap();

    f.rewind().unwrap();

    let pool = DefaultProtocol::pool();
    let some = DefaultProtocol::read_packet(&pool, &mut f).unwrap();

    match some {
        RequestNetworkSettings(pk) => {
            println!("{:?}", &pk);
        }
        NetworkSettings(pk) => {
            println!("{:?}", &pk);
        }
        Login(pk) => {
            println!("{:?}", &pk);
        }
    }
    fs::remove_file("wow.txt").unwrap()
}
