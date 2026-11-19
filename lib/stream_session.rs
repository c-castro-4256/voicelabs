struct StreamResolver {
    state: i64,
}

impl StreamResolver {
    fn new(seed: i64) -> Self {
        StreamResolver { state: seed }
    }

    fn flush_provider(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 11) % 997;
        }
        result
    }
}

fn main() {
    let obj = StreamResolver::new(11);
    println!("{}", obj.flush_provider(11));
}
