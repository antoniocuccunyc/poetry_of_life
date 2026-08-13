const WALL_WIDTH: usize = 20;
const WALL_HEIGHT: usize = 20;

const PATCH_WIDTH: usize = 4;
const PATCH_HEIGHT: usize = 2;

const GRID_WIDTH: usize = WALL_WIDTH * PATCH_WIDTH;    // 80
const GRID_HEIGHT: usize = WALL_HEIGHT * PATCH_HEIGHT; // 40

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Dead,
    Alive,
}

struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width * height];
        let mut universe = Universe { width, height, cells };
        universe.seed_r_pentomino();
        universe
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
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

    fn tick(&mut self) {
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

    fn render(&self) -> String {
        let mut out = String::new();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let symbol = if self.cells[idx] == Cell::Alive {
                    '◼'
                } else {
                    '◻'
                };
                out.push(symbol);
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

    fn patch_byte(&mut self, wall_row: usize, wall_col:usize) -> u8 {
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
}
fn main() {
    let mut universe = Universe::new(32, 16);
    print!("{}", universe.render());
    for generation in 0..5 {
        println!("Generation {}:", generation);
        print!("{}", universe.render());
        println!();
        universe.tick();
    }
}