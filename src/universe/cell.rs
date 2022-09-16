use std::fmt;

#[derive(Clone, Copy)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Dead => write!(f, "◻"),
            Cell::Alive => write!(f, "◼"),
        }
    }
}
