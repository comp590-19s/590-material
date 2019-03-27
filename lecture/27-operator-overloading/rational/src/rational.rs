use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    n: i64,
    d: i64,
}

impl Rational {
    pub fn from(numerator: i64, denominator: i64) -> Rational {
        Rational {
            n: numerator,
            d: denominator,
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

// Euclid's greatest common divisor Algorithm
fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
