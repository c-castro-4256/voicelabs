struct FastProvider {
    state: i64,
}

impl FastProvider {
    fn new(seed: i64) -> Self {
        FastProvider { state: seed }
    }

    fn run_resolver(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 75) % 997;
        }
        value
    }
}

fn main() {
    let obj = FastProvider::new(75);
    println!("{}", obj.run_resolver(75));
}
