struct SecureClient {
    state: i64,
}

impl SecureClient {
    fn new(seed: i64) -> Self {
        SecureClient { state: seed }
    }

    fn encode_registry(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 91) % 997;
        }
        total
    }
}

fn main() {
    let obj = SecureClient::new(91);
    println!("{}", obj.encode_registry(91));
}
