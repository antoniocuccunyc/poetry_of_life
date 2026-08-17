use rand::rngs::StdRng;
use rand::Rng;

use crate::glyph::glyph;
use crate::{PATCH_HEIGHT, PATCH_WIDTH, WALL_HEIGHT, WALL_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width * height];
        let mut universe = Universe { width, height, cells };
        universe.seed_r_pentomino();
        universe
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn seed_uniform(&mut self, rng: &mut StdRng, density: f64) {
        for idx in 0..self.cells.len() {
            if rng.gen_bool(density) {
                self.cells[idx] = Cell::Alive;
            }
        }
    }

    pub fn seed_blob(&mut self, rng: &mut StdRng, size: usize, density: f64) {
        let top = self.height / 2 - size / 2;
        let left = self.width / 2 - size / 2;

        for r in 0..size {
            for c in 0..size {
                if rng.gen_bool(density) {
                    let idx = self.get_index((top + r) % self.height, (left + c) % self.width);
                    self.cells[idx] = Cell::Alive;
                }
            }
        }
    }

    pub fn seed_scattered(&mut self, rng: &mut StdRng, count: usize, size: usize, density: f64) {
        for _ in 0..count {
            let top = rng.gen_range(0..self.height);
            let left = rng.gen_range(0..self.width);

            for r in 0..size {
                for c in 0..size {
                    if rng.gen_bool(density) {
                        let idx = self.get_index((top + r) % self.height, (left + c) % self.width);
                        self.cells[idx] = Cell::Alive;
                    }
                }
            }
        }
    }

    fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);

                count += match self.cells[idx] {
                    Cell::Alive => 1,
                    Cell::Dead => 0,
                };
            }
        }

        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                next[idx] = match (cell, live_neighbors) {
                    (Cell::Alive, n) if n < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, n) if n > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cells = next;
    }

    pub fn render(&mut self) -> String {
        let mut out = String::with_capacity(self.width * (self.height + 1));

        for row in 0..self.height {
            for column in 0..self.width {
                out.push(glyph(self.patch_byte(row, column)));
            }
            out.push('\n');
        }

        out
    }

    fn set_alive(&mut self, coords: &[(usize, usize)]) {
        for &(row, col) in coords {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn patch_byte(&mut self, wall_row: usize, wall_col: usize) -> u8 {
        let top = wall_row * PATCH_HEIGHT;
        let left = wall_col * PATCH_WIDTH;
        let mut byte = 0u8;
        for row in 0..PATCH_HEIGHT {
            for col in 0..PATCH_WIDTH {
                let idx = self.get_index((top + row) % self.height, (left + col) % self.width);
                byte <<= 1;
                if self.cells[idx] == Cell::Alive {
                    byte |= 1;
                }
            }
        }
        byte
    }

    fn seed_r_pentomino(&mut self) {
        let r = self.height / 2;
        let c = self.width / 2;
        self.set_alive(&[
            (r, c + 1),
            (r, c + 2),
            (r + 1, c),
            (r + 1, c + 1),
            (r + 2, c + 1),
        ])
    }

    pub fn wall_values(&mut self) -> Vec<u8> {
        let mut values = Vec::with_capacity(WALL_WIDTH * WALL_HEIGHT);
        for row in 0..WALL_HEIGHT {
            for col in 0..WALL_WIDTH {
                values.push(glyph(self.patch_byte(row, col)) as u8 - 32);
            }
        }
        values
    }
}
