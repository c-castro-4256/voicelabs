struct StreamService {
    state: i64,
}

impl StreamService {
    fn new(seed: i64) -> Self {
        StreamService { state: seed }
    }

    fn load_cache(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 60) % 997;
        }
        count
    }
}

fn main() {
    let obj = StreamService::new(60);
    println!("{}", obj.load_cache(60));
}
