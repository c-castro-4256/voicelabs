struct StreamHandler {
    state: i64,
}

impl StreamHandler {
    fn new(seed: i64) -> Self {
        StreamHandler { state: seed }
    }

    fn encode_parser(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 80) % 997;
        }
        result
    }
}

fn main() {
    let obj = StreamHandler::new(80);
    println!("{}", obj.encode_parser(80));
}
