use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;

use crate::minecraft::packets::PacketKind::RequestNetworkSettings;
use crate::minecraft::packets::{NetworkSettingsPacket, PacketKind};

mod minecraft;

fn main() {
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open("wow.txt")
        .unwrap();

    let mut pk = NetworkSettingsPacket::default();
    pk.compression_algorithm = 1;
    println!("{:?}", &pk);

    minecraft::write_packet(&mut f, &pk).unwrap();

    f.rewind().unwrap();

    let some = minecraft::read_packet(&mut f).unwrap();
    match some {
        RequestNetworkSettings(pk) => {
            println!("{:?}", &pk);
        }
        PacketKind::NetworkSettings(pk) => {
            println!("{:?}", &pk);
        }
    }
    fs::remove_file("wow.txt").unwrap()
}
