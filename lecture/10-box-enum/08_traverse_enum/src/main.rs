#[derive(Debug)]
enum NGram {
    Node { data: char, next: Box<NGram> },
    End,
}

use self::NGram::{End, Node};

fn main() {
    let b = Node {
        data: 'b',
        next: Box::new(End),
    };
    let a = Node {
        data: 'a',
        next: Box::new(b),
    };
    println!("iterative: {}", traverse(&a));
    println!("recursive: {}", traverse_recur(&a));
}

fn traverse(ngram: &NGram) -> String {
    let mut result = String::new();
    let mut cursor = ngram;
    while let Node { data, next } = cursor {
        result.push_str(&format!("{} -> ", data));
        cursor = &next;
    }
    result.push_str("End");
    result
}

fn traverse_recur(ngram: &NGram) -> String {
    match ngram {
        Node { data, next } => format!("{} -> {}", data, traverse_recur(&next)),
        End => String::from("End"),
    }
}
