#[derive(Debug)]
enum Value {
    Number(f64),
    Pair(Box<Value>, Box<Value>),
    End
}
use self::Value::{Number, Pair, End};

fn cons(first: Value, rest: Value) -> Value {
    Pair(Box::new(first), Box::new(rest))
}

fn to_dot(val: Value) -> String {
    let dot = String::from("digraph {\n");
    dot.push_str(traverse(val));
    dot.push_str("}");
    dot
}

fn traverse(val: Value) -> String {
    let res = String::new();
    match val {
        Number(val) => res.push_str(format!("\t{}\n", val)),
        Pair(lhs, rhs) => {
            res.push_str("node_0 [label=\"{}\"]")
        }
        End => { /* noop */ }
    }
    res
}

fn main() {
    let x = Number(64.0);
    let y = cons(x, End);
    println!("{:?}", y);
}
