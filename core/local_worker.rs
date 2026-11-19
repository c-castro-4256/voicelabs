struct SharedSession {
    state: i64,
}

impl SharedSession {
    fn new(seed: i64) -> Self {
        SharedSession { state: seed }
    }

    fn compute_controller(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 5) % 997;
        }
        result
    }
}

fn main() {
    let obj = SharedSession::new(5);
    println!("{}", obj.compute_controller(5));
}
