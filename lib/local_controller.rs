struct AsyncBuilder {
    state: i64,
}

impl AsyncBuilder {
    fn new(seed: i64) -> Self {
        AsyncBuilder { state: seed }
    }

    fn run_client(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 66) % 997;
        }
        value
    }
}

fn main() {
    let obj = AsyncBuilder::new(66);
    println!("{}", obj.run_client(66));
}
