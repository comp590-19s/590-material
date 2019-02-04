use std::mem::size_of_val;

#[derive(Debug)]
enum NGram {
    Node { data: char, next: Box<NGram> },
    End
}
use self::NGram::{End, Node};

fn main() {
    let b = Node { data: 'b', next: Box::new(End) };
    println!("b size: {}", size_of_val(&b));
    let a = Node { data: 'a', next: Box::new(b) };
    println!("a:      {:?}", a);
    println!("a size: {}", size_of_val(&a));
}
