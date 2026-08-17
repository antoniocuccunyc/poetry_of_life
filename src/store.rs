use std::fs;

struct ChunkStore {
    one: Vec<&'static str>,
    two: Vec<&'static str>,
    three: Vec<&'static str>,
}

impl ChunkStore {
    fn load(path: &str) -> ChunkStore {
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

    fn bucket(&self, n: usize) -> &[&'static str] {
        match n {
            1 => &self.one,
            2 => &self.two,
            _ => &self.three,
        }
    }


}

