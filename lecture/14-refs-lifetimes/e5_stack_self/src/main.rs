struct Counter {
    x: u64,
}

impl Counter {
    fn from(x: u64) -> Counter {
        Counter { x }
    }

    fn incr(&mut self) {
        self.x += 1;
        // Pause here!
    }

    fn get(&self) -> u64 {
        self.x
    }
}

fn main() {
    let mut c = Counter::from(0);
    c.incr();
    println!("{}", c.get());
}


