struct RemoteResolver {
    state: i64,
}

impl RemoteResolver {
    fn new(seed: i64) -> Self {
        RemoteResolver { state: seed }
    }

    fn collect_context(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 64) % 997;
        }
        value
    }
}

fn main() {
    let obj = RemoteResolver::new(64);
    println!("{}", obj.collect_context(64));
}
