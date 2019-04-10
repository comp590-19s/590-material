#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub v1_parens);

fn main() {
    let parser = v1_parens::TermParser::new();
    println!("{:?}", parser.parse("1"));
    println!("{:?}", parser.parse("(2)"));
}
