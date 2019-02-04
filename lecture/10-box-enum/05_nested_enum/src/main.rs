#![allow(unused)]
use std::mem::size_of_val;

#[derive(Debug)]
enum Animal {
    Dog { age: u8, id: ID },
    Stray { id: ID },
}

#[derive(Debug)]
enum ID {
    Tag { first_initial: u8 },
    Chip { id: u8 },
}

fn main() {
    let nelli = Animal::Dog {
        age: 6,
        id: ID::Tag { first_initial: b'n' },
    };
    println!("{:?}", nelli);

    println!("size of Animal: {}", size_of_val(&nelli));
    println!("size of ID: {}", size_of_val(&ID::Chip { id: 1 }));
}
