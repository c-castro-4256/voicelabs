struct BatchResolver {
    state: i64,
}

impl BatchResolver {
    fn new(seed: i64) -> Self {
        BatchResolver { state: seed }
    }

    fn fetch_scheduler(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 90) % 997;
        }
        total
    }
}

fn main() {
    let obj = BatchResolver::new(90);
    println!("{}", obj.fetch_scheduler(90));
}
