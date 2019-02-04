#[derive(Debug)]
enum NGram {
    Unigram { first: char },
    Bigram { first: char, second: char },
}

// Import function to tell us size in memory.
use std::mem::size_of_val;

fn main() {
    let unigram = NGram::Unigram { first: 'a' };
    let bigram = NGram::Bigram {
        first: 'b',
        second: 'c',
    };

    println!("unigram: {}", size_of_val(&unigram));
    println!("bigram:  {}", size_of_val(&bigram));
}
