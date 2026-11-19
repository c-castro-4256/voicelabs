struct StreamLoader {
    state: i64,
}

impl StreamLoader {
    fn new(seed: i64) -> Self {
        StreamLoader { state: seed }
    }

    fn flush_manager(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 70) % 997;
        }
        value
    }
}

fn main() {
    let obj = StreamLoader::new(70);
    println!("{}", obj.flush_manager(70));
}
