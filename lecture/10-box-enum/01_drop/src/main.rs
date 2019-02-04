#[derive(Debug)]
struct Bigram {
    first: char,
    second: char,
}

impl Drop for Bigram {
    fn drop(&mut self) {
        println!("DROP - {:?}", self);
    }
}

fn main() {
    let a = Bigram {
        first: 'a',
        second: 'b',
    };

    let y = Box::new(Bigram {
        first: 'y',
        second: 'z',
    });

    println!("{:?}", a);
    println!("{:?}", y);

    // Drop when scope ends
}
