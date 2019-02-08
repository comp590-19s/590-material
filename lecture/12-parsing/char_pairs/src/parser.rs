use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Node {
    Char(char),
    Pair(Box<Node>, Box<Node>),
}

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn new(tokens: Tokenizer) -> Parser {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_expr(&mut self) -> Node {
        if let Some(token) = self.tokens.peek() {
            match *token {
                Token::Char(c) => return self.parse_char(c),
                Token::LParen => return self.parse_pair(),
                _ => { /* Fall through to panic */ }
            }
        }
        panic!("parse_expr: Expected Char | Pair");
    }

    fn parse_pair(&mut self) -> Node {
        self.take(Token::LParen);
        let lhs = self.parse_expr();
        self.take(Token::Space);
        let rhs = self.parse_expr();
        self.take(Token::RParen);
        Node::Pair(Box::new(lhs), Box::new(rhs))
    }

    fn parse_char(&mut self, c: char) -> Node {
        self.take(Token::Char(c));
        Node::Char(c)
    }

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
