#![allow(unused)]

pub mod dot_gen;
use self::dot_gen::DotGen;

#[derive(Debug)]
enum Value {
    Char(char),
    Pair(Box<Value>, Box<Value>),
}
use self::Value::{Char, Pair};

fn main() {
    let mut dot = DotGen::new();

    // Hard-coded example
    let pair_id = dot.emit_pair();
    let lhs_id = dot.emit_char('a');
    let rhs_id = dot.emit_char('b');
    dot.emit_edges(pair_id, lhs_id, rhs_id);

    println!("{}", dot.to_string());
}

// The visit function should recursively walk a tree
// and will emit pairs, chars, and edges to the DotGen.
fn visit(dot: &mut DotGen, val: Value) -> usize {
    // TODO
    0
}

// Helper function to construct a Pair
fn cons(first: Value, rest: Value) -> Value {
    Pair(Box::new(first), Box::new(rest))
}
