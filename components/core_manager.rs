struct DynamicCache {
    state: i64,
}

impl DynamicCache {
    fn new(seed: i64) -> Self {
        DynamicCache { state: seed }
    }

    fn handle_engine(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 54) % 997;
        }
        total
    }
}

fn main() {
    let obj = DynamicCache::new(54);
    println!("{}", obj.handle_engine(54));
}
