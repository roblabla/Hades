use std::{thread, time::Duration};
use hs_gba::GBA;

fn main() {
    let mut gba = GBA::new();

    thread::spawn(move || {
        gba.run();
    });

    thread::sleep(Duration::from_secs(1));
}