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
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
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
}
fn main() {
    let universe = Universe::new(32, 16);
    print!("{}", universe.render());
}