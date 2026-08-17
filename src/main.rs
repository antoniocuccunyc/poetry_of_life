mod glyph;
mod island;
mod universe;
mod store;

use rand::prelude::StdRng;
use rand::SeedableRng;
use island::find_islands;
use universe::Universe;

const WALL_WIDTH: usize = 20;
const WALL_HEIGHT: usize = 20;

const PATCH_WIDTH: usize = 4;
const PATCH_HEIGHT: usize = 2;

const GRID_WIDTH: usize = WALL_WIDTH * PATCH_WIDTH;    // 80
const GRID_HEIGHT: usize = WALL_HEIGHT * PATCH_HEIGHT; // 40

fn main() {
    for seed in [1u64, 7, 42, 1337, 99999] {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut universe = Universe::new(GRID_WIDTH, GRID_HEIGHT);
        universe.seed_blob(&mut rng, 20, 0.4);

        println!("═══ seed {} ═══", seed);

        let mut sizes = Vec::new();
        let mut means = Vec::new();
        let mut splits = 0;
        let mut previous_size = 0;

        for generation in 0..80 {
            universe.tick();

            let values = universe.wall_values();
            let mut islands = find_islands(&values);
            islands.sort_by(|a, b| b.size().cmp(&a.size()));

            if let Some(big) = islands.first() {
                let size = big.size();
                let mean = big.sum / size;

                // A sharp drop in the largest island usually means it split.
                if previous_size > 0 && size * 2 < previous_size {
                    splits += 1;
                }
                previous_size = size;

                sizes.push(size);
                means.push(mean);

                if generation % 10 == 0 {
                    println!(
                        "  gen {:>3}  islands {:>2}  size {:>3}  sum {:>5}  mean {:>3}",
                        generation,
                        islands.len(),
                        size,
                        big.sum,
                        mean
                    );
                }
            }
        }

        println!(
            "  → size peak {}, mean range {}–{}, sharp drops {}\n",
            sizes.iter().max().unwrap_or(&0),
            means.iter().min().unwrap_or(&0),
            means.iter().max().unwrap_or(&0),
            splits
        );
    }
}