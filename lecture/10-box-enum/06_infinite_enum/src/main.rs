use std::mem::size_of_val;

enum NGram {
    Node { data: char, next: NGram },
    End
}
use self::NGram::{Node, End};

fn main() {
    let b = Node { data: 'b', next: End };
    let a = Node { data: 'a', next: b };
    println!("a: {}", size_of_val(&a));
}
