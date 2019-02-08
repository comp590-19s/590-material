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

    // Top-level, public parse entry point to parse a CharPair Value.
    // Grammar: Value -> Char | Pair
    pub fn parse_value(&mut self) -> Value {
        if let Some(token) = self.tokens.peek() {
            match *token {
                // TODO #2: Match *first* token of Char or Pair
                _ => { /* Fall through to panic */ }
            }
        }
        panic!("parse_value: Expected Char | Pair");
    }

    // TODO #1) Define methods for non-terminals of grammar.
    
    // Grammar: Char -> Any Token::Char
    
    // Grammar: Pair -> LParen Value Space Value RParen


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
