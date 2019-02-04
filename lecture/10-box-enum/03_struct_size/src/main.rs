#[derive(Debug)]
struct Bigram {
    first: char,
    second: char,
}

// Import function to tell us size in memory.
use std::mem::size_of_val;

fn main() {
    let a = 'a';
    let b = Bigram {
        first: 'b',
        second: 'c',
    };

    println!("{}", size_of_val(&a));
    println!("{}", size_of_val(&b));
}
