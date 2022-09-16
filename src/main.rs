use std::{thread, time};
mod universe;

fn sleep(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

fn main() {
    let mut universe = universe::Universe::new(30);
    loop {
        universe.tick();
        println!("{esc}c{u}", esc = 27 as char, u = universe);
        sleep(400);
    }
}
