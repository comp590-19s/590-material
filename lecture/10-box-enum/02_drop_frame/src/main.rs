#[derive(Debug)]
struct Bigram {
    first: char,
    second: char,
}

impl Drop for Bigram {
    fn drop(&mut self) {
        println!("DROP      {:?}", self);
    }
}

fn main() {
    let a = bigram_builder();
    println!("main a:   {:?}", a);
}

fn bigram_builder() -> Bigram {
    let local1 = Bigram {
        first: 'a',
        second: 'b',
    };
    println!("New:      {:?}", local1);

    let local2 = Bigram {
        first: 'x',
        second: 'y',
    };
    println!("New:      {:?}", local2);

    local2
}
