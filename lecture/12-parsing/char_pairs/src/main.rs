#![allow(unused)]

use std::io;
use std::iter::Peekable;

pub mod dot_gen;
pub mod parser;
pub mod tokenizer;

use self::dot_gen::DotGen;
use self::parser::Parser;
use self::tokenizer::Tokenizer;

fn main() {
    let source = read_line();
    let parse_tree = Parser::new(Tokenizer::new(&source)).parse_expr();
    let target = DotGen::new(&parse_tree).to_string();
    println!("{}", target);
}

/**
 * Read a line of input from the standard input.
 */
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input
}
