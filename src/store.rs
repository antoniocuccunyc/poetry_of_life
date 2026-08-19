use std::fs;
const DIVISOR: usize = 100_000;
use crate::island::Island;
pub struct ChunkStore {
    one: Vec<&'static str>,
    two: Vec<&'static str>,
    three: Vec<&'static str>,
}

impl ChunkStore {
    pub fn load(path: &str) -> ChunkStore {
        let text = fs::read_to_string(path).expect("could not read text file");
        let text: &'static str = Box::leak(text.into_boxed_str());

        let mut store = ChunkStore {
            one: Vec::new(),
            two: Vec::new(),
            three: Vec::new(),
        };
        for line in text.lines() {
            let chunk = line.trim();
            if chunk.is_empty() {
                continue;
            }
            match chunk.split_whitespace().count() {
                1 => store.one.push(chunk),
                2 => store.two.push(chunk),
                3 => store.three.push(chunk),
                _ => {}
            }
    }

    store
    }

    pub fn pick(&self, island_count: usize, sum: usize) -> &'static str {
        let words = match island_count {
            0..=2 => 3,
            3..=5 => 2,
            _ => 1,
        };

        let bucket = self.bucket(words);
        if bucket.is_empty() {
            return "";
        }

        bucket[(sum / DIVISOR) % bucket.len()]
    }

    fn bucket(&self, n: usize) -> &[&'static str] {
        match n {
            1 => &self.one,
            2 => &self.two,
            _ => &self.three,
        }
    }




}

pub fn compose_line(islands: &[Island], store: &ChunkStore) -> String {
    let mut ranked: Vec<&Island> = islands.iter().collect();
    ranked.sort_by(|a, b| b.size().cmp(&a.size()));

    let mut chunks = Vec::new();

    for island in ranked.iter().take(2) {
        let chunk = store.pick(islands.len(), island.sum);

        if !chunk.is_empty() {
            chunks.push(chunk);
        }
    }

    chunks.join(" ")
}