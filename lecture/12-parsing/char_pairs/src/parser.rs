use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Value {
    Char(char),
    Pair(Box<Value>, Box<Value>),
}

// The Parser's internal state is just a Peekable Tokenizer
pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn new(tokens: Tokenizer) -> Parser {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_value(&mut self) -> Value {
        if let Some(token) = self.tokens.peek() {
            match *token {
                Token::Char(c) => return self.parse_char(c),
                Token::LParen => return self.parse_pair(),
                _ => { /* Fall through to panic */ }
            }
        }
        panic!("parse_value: Expected Char | Pair");
    }

    fn parse_char(&mut self, c: char) -> Value {
        self.take(Token::Char(c));
        Value::Char(c)
    }

    fn parse_pair(&mut self) -> Value {
        self.take(Token::LParen);
        let lhs = self.parse_value();
        self.take(Token::Space);
        let rhs = self.parse_value();
        self.take(Token::RParen);
        Value::Pair(Box::new(lhs), Box::new(rhs))
    }

    // Helper function to "take" the next token from the tokens iterator
    // and ensure that it is exactly what we expected. If not, we'll panic.
    fn take(&mut self, expected: Token) {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                panic!("Expected: {:?} - Found {:?}", expected, next);
            }
        } else {
            panic!("Expected: {:?} - Found None", expected);
        }
    }
}
