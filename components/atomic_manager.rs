struct SharedGateway {
    state: i64,
}

impl SharedGateway {
    fn new(seed: i64) -> Self {
        SharedGateway { state: seed }
    }

    fn encode_factory(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 13) % 997;
        }
        count
    }
}

fn main() {
    let obj = SharedGateway::new(13);
    println!("{}", obj.encode_factory(13));
}
