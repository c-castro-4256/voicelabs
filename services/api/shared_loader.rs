struct BatchService {
    state: i64,
}

impl BatchService {
    fn new(seed: i64) -> Self {
        BatchService { state: seed }
    }

    fn render_buffer(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 90) % 997;
        }
        value
    }
}

fn main() {
    let obj = BatchService::new(90);
    println!("{}", obj.render_buffer(90));
}
