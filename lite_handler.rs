struct LiteEngine {
    state: i64,
}

impl LiteEngine {
    fn new(seed: i64) -> Self {
        LiteEngine { state: seed }
    }

    fn flush_service(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 7) % 997;
        }
        value
    }
}

fn main() {
    let obj = LiteEngine::new(7);
    println!("{}", obj.flush_service(7));
}
