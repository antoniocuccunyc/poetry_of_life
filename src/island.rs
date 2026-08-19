use std::collections::VecDeque;

use crate::{WALL_HEIGHT, WALL_WIDTH};

#[derive(Debug)]
pub struct Island {
    pub cells: Vec<(usize, usize)>,
    pub sum: usize,
}

impl Island {
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    /// Topmost-then-leftmost cell, used for reading order.
    pub fn anchor(&self) -> (usize, usize) {
        self.cells[0]
    }
}

/// Flood-fills the wall-glyph grid (toroidal, 4-neighbour) into connected
/// islands of non-zero values.
pub fn find_islands(values: &[u32]) -> Vec<Island> {
    let mut labelled = vec![false; values.len()];
    let mut islands = Vec::new();
    for start in 0..values.len() {
        if values[start] == 0 || labelled[start] {
            continue;
        }
        let mut cells = Vec::new();
        let mut sum = 0usize;
        let mut queue = VecDeque::new();

        labelled[start] = true;
        queue.push_back(start);
        while let Some(idx) = queue.pop_front() {
            let row = idx / WALL_WIDTH;
            let col = idx % WALL_WIDTH;
            cells.push((row, col));
            sum += values[idx] as usize;

            let neighbours = [
                (WALL_HEIGHT - 1, 0),
                (1, 0),
                (0, WALL_WIDTH - 1),
                (0, 1),
            ];

            for (delta_row, delta_col) in neighbours {
                let n_row = (row + delta_row) % WALL_HEIGHT;
                let n_col = (col + delta_col) % WALL_WIDTH;
                let n_idx = n_row * WALL_WIDTH + n_col;

                if values[n_idx] != 0 && !labelled[n_idx] {
                    labelled[n_idx] = true;
                    queue.push_back(n_idx);
                }
            }
        }
        islands.push(Island { cells, sum });
    }

    islands
}
