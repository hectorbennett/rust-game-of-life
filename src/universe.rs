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

        let seed1 = 3;
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbour_count = self.get_live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbour_count) {
                    (cell::Cell::Alive, x) if x < 2 => cell::Cell::Dead,
                    (cell::Cell::Alive, 2) | (cell::Cell::Alive, 3) => cell::Cell::Alive,
                    (cell::Cell::Alive, x) if x > 3 => cell::Cell::Dead,
                    (cell::Cell::Dead, 3) => cell::Cell::Alive,
                    (otherwise, _) => otherwise
                };
                next[index] = next_cell;
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
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
