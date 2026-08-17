mod glyph;
mod island;
mod store;
mod universe;

use rand::prelude::StdRng;
use rand::SeedableRng;

use island::find_islands;
use store::{compose_line, ChunkStore};
use universe::Universe;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const WALL_WIDTH: usize = 20;
const WALL_HEIGHT: usize = 20;

const PATCH_WIDTH: usize = 4;
const PATCH_HEIGHT: usize = 2;

const GRID_WIDTH: usize = WALL_WIDTH * PATCH_WIDTH; // 80
const GRID_HEIGHT: usize = WALL_HEIGHT * PATCH_HEIGHT; // 40

const GENERATIONS: usize = 200;
const SEED: u64 = 42;

/// Set to false to advance one generation per Enter key instead of on a timer.
const ANIMATE: bool = false;
const FRAME_SECONDS: u64 = 5;

/// How many identical lines in a row before the poem is considered frozen.
const FREEZE_AFTER: usize = 3;

const CLEAR: &str = "\x1B[2J\x1B[H";
const RED: &str = "\x1B[31m";
const RESET: &str = "\x1B[0m";

fn main() {
    let store = ChunkStore::load("data/chunks.txt");

    let mut rng = StdRng::seed_from_u64(SEED);
    let mut universe = Universe::new(GRID_WIDTH, GRID_HEIGHT);
    universe.seed_blob(&mut rng, 20, 0.4);

    let mut previous_line = String::new();
    let mut repeats = 0;

    for _ in 0..GENERATIONS {
        universe.tick();

        let values = universe.wall_values();
        let islands = find_islands(&values);
        let line = compose_line(&islands, &store);

        if line == previous_line {
            repeats += 1;
        } else {
            repeats = 0;
            previous_line = line.clone();
        }

        print!("{}", CLEAR);
        print!("{}", universe.render_wall());
        println!();

        if repeats >= FREEZE_AFTER {
            println!("{}{}{}", RED, line, RESET);
        } else {
            println!("{}", line);
        }

        io::stdout().flush().unwrap();

        if ANIMATE {
            thread::sleep(Duration::from_secs(FRAME_SECONDS));
        } else {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
    }
}