use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;

use minecraft::packets::RequestNetworkSettingsPacket;

use crate::minecraft::packets::PacketKind::RequestNetworkSettings;

mod minecraft;

fn main() {
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open("wow.txt")
        .unwrap();

    let pk = RequestNetworkSettingsPacket { protocol: 112 };
    println!("{:?}", &pk);

    minecraft::write_packet(&mut f, &pk).unwrap();

    f.rewind().unwrap();

    let some = minecraft::read_packet(&mut f).unwrap();
    match some {
        RequestNetworkSettings(pk) => {
            println!("{:?}", &pk);
        }
    }
    fs::remove_file("wow.txt").unwrap()
}