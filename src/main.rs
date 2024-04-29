use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;

use bstream::Vu32LenByteSlice;

use crate::minecraft::*;
use crate::minecraft::packets::LoginPacket;
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

    let pk = LoginPacket {
        client_protocol: DefaultProtocol::id(),
        connection_request: Vu32LenByteSlice::from("{}{}{}"),
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
