use std::str::Chars;

/*
 * A very simplistic tokenizer.
 */

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Space,
    Char(char),
}

pub struct Tokenizer<'str> {
    chars: Chars<'str>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars(),
        }
    }
}

impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.chars.next() {
            if c == '\n' {
                None
            } else {
                Some(match c {
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    ' ' => Token::Space,
                    _ => Token::Char(c),
                })
            }
        } else {
            None
        }
    }
}
