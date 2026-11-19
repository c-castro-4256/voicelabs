struct AtomicParser {
    state: i64,
}

impl AtomicParser {
    fn new(seed: i64) -> Self {
        AtomicParser { state: seed }
    }

    fn decode_processor(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 29) % 997;
        }
        value
    }
}

fn main() {
    let obj = AtomicParser::new(29);
    println!("{}", obj.decode_processor(29));
}
