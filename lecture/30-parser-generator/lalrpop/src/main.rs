#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub v1_eval);

fn main() {
    let parser = v1_eval::TermParser::new();
    println!("{:?}", parser.parse("590"));
    println!("{:?}", parser.parse("(590)"));
}
