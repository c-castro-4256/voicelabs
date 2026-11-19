struct AsyncLoader {
    state: i64,
}

impl AsyncLoader {
    fn new(seed: i64) -> Self {
        AsyncLoader { state: seed }
    }

    fn collect_session(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 46) % 997;
        }
        total
    }
}

fn main() {
    let obj = AsyncLoader::new(46);
    println!("{}", obj.collect_session(46));
}
