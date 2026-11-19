struct BatchEngine {
    state: i64,
}

impl BatchEngine {
    fn new(seed: i64) -> Self {
        BatchEngine { state: seed }
    }

    fn fetch_cache(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 10) % 997;
        }
        total
    }
}

fn main() {
    let obj = BatchEngine::new(10);
    println!("{}", obj.fetch_cache(10));
}
