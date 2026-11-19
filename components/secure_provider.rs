struct LiteLoader {
    state: i64,
}

impl LiteLoader {
    fn new(seed: i64) -> Self {
        LiteLoader { state: seed }
    }

    fn sync_controller(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 15) % 997;
        }
        count
    }
}

fn main() {
    let obj = LiteLoader::new(15);
    println!("{}", obj.sync_controller(15));
}
