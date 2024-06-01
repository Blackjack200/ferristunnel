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

    let mut ma = statem::StateMachine::new(1i32);
    ma.entry(2, |&_old| {
        println!("enter 2");
    });
    ma.exit(2, |&_old| {
        println!("exit 2");
    });
    ma.entry(3, |&_old| {
        println!("enter 3");
    });
    ma.exit(3, |&_old| {
        println!("exit 3");
    });
    ma.permit(1, vec![2]);
    ma.permit(2, vec![3]);
    ma.permit(3, vec![1]);
    ma.fire(2);
    ma.fire(3);
    ma.fire(1);

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
        _ => {}
    }
    fs::remove_file("wow.txt").unwrap()
}
