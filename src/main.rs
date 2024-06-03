use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::Seek;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use piston_window::{
    clear, PistonWindow, rectangle, RenderEvent, Transformed, UpdateEvent, WindowSettings,
};

use physics_discrete::{interpolate, Object, Space};
use physics_discrete::interpolate::Interpolator;
use physics_discrete::minecraft::{MinecraftSpace, MovingEntity};

use crate::minecraft::*;
use crate::minecraft::packets::{CompressionAlgorithm, NetworkSettingsPacket};
use crate::minecraft::packets::PacketKind::*;

mod minecraft;

fn main() {
    test_state_machine();
    test_protocol();
    test_entity();
}

fn test_protocol() {
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
        _ => {}
    }
    fs::remove_file("wow.txt").unwrap()
}

fn test_state_machine() {
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
}

type ContinuousMeasure = <MinecraftSpace as Space>::ContinuousMeasure;
type Position = <MinecraftSpace as Space>::Position;
const INITIAL_X: ContinuousMeasure = 100.0;
const INITIAL_Y: ContinuousMeasure = 400.0;
const GROUND_Y: ContinuousMeasure = 0.0;
const GRAVITY: ContinuousMeasure = 20.0;

fn test_entity() {
    let entity = Arc::new(Mutex::new(MovingEntity::<MinecraftSpace>::default()));
    let entity_clone = entity.clone();
    {
        let mut ent = entity.lock().unwrap();
        let pos = ent.position();
        pos.x = INITIAL_X;
        pos.y = INITIAL_Y;
    }
    let last_update = Arc::new(Mutex::new(Instant::now()));
    let last_update_clone = last_update.clone();

    thread::spawn(move || loop {
        {
            let mut ent = entity_clone.lock().unwrap();
            ent.end_previous_tick();
            if ent.position().y <= GROUND_Y {
                let pos = ent.position();
                pos.x = INITIAL_X;
                pos.y = INITIAL_Y;
            }
            ent.delta_position().y -= GRAVITY;
        }
        {
            let mut em = last_update.lock().unwrap();
            *em = Instant::now();
        }
        thread::sleep(Duration::from_secs_f64(MinecraftSpace::per_tick() as f64));
    });

    let mut window: PistonWindow = WindowSettings::new("Movement Simulation", [800, 600])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                clear([1.0; 4], g);
                {
                    let pos: Position;
                    let delta_pos: Position;
                    let t: f64;
                    {
                        let mut entity = entity.lock().unwrap();
                        t = last_update_clone.lock().unwrap().elapsed().as_secs_f64();
                        pos = *entity.position();
                        delta_pos = *entity.delta_position();
                    }
                    let interpolated_pos =
                        interpolate::LinearInterpolator::interpolate::<Position, MinecraftSpace>(
                            pos,
                            pos + delta_pos,
                            MinecraftSpace::per_tick(),
                            t as ContinuousMeasure,
                        );
                    let square = rectangle::square(0.0, 0.0, 50.0);

                    let transform = c.transform.trans(pos.x as f64, 600.0 - (pos.y as f64));
                    rectangle([0.0, 0.0, 1.0, 0.5], square, transform, g);

                    let pos = interpolated_pos;
                    let transform = c.transform.trans(pos.x as f64, 600.0 - (pos.y as f64));
                    rectangle([1.0, 0.0, 0.0, 1.0], square, transform, g);
                }
            });
        }
    }
}
