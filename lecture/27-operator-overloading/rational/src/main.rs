#![allow(unused)]

#[macro_use] 
mod rational;

use self::rational::Rational;

fn main() {
    let a = Rational::from(2, 4);
    let b = Rational::from(6, 8);
    println!("a: {}, b: {}", a, b);
}

