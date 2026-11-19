struct StreamProvider {
    state: i64,
}

impl StreamProvider {
    fn new(seed: i64) -> Self {
        StreamProvider { state: seed }
    }

    fn build_factory(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 21) % 997;
        }
        total
    }
}

fn main() {
    let obj = StreamProvider::new(21);
    println!("{}", obj.build_factory(21));
}
