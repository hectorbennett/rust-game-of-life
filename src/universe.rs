use std::fmt;
pub mod cell;

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<cell::Cell>,
}

impl Universe {
    pub fn new(size: u32) -> Universe {
        println!("new universe");
        let width = size;
        let height = size;

        let seed1 = 2;
        let seed2 = 7;

        /* Generate random start for cells */
        let cells = (0..width * height)
            .map(|i| {
                if i % seed1 == 0 || i % seed2 == 0 {
                    cell::Cell::Alive
                } else {
                    cell::Cell::Dead
                }
            })
            .collect();
        Universe {
            width: width,
            height: height,
            cells: cells,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "This is the universe")
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for c in line {
                // println!("{}", c);
                // let symbol = " ◻ ";
                // let symbol = if c == cell::Cell::Dead { '◻' } else { '◼' };
                write!(f, " {} ", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
