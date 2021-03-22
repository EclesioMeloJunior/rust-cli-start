use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbours_count(row, col);

                next[idx] = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise
                }
            }
        }

        self.cells = next;
    }

    fn live_neighbours_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_row == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;

                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn row_as_string(&self, row: u32) -> Option<String> {
        if row < self.height {
            let mut row_string = String::new();
            let start = self.get_index(row, 0);
            let end = self.get_index(row, self.width);

            let line = &self.cells[start..end];
            for &cell in line {
                let symbol = if cell == Cell::Dead { 'o' } else { 'x' };
                row_string.push(symbol)
            }
            Some(row_string)
        } else {
            None
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { 'o' } else { 'x' };
                write!(f, "{}", symbol)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}