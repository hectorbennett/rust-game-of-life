// turn off some lint warnings while developing
#![allow(unused_variables)]
#![allow(dead_code)]

mod universe;
use universe::cell;

fn main() {
    println!("Hello, world!");
    let universe = universe::Universe::new(20);
    // let display = format!("{}", universe);
    println!("{}", universe);
    // let dead_cell = cell::Cell::Dead;
    // let alive_cell = cell::Cell::Alive;
    // println!("{}", dead_cell);
    // println!("{}", alive_cell);
}
