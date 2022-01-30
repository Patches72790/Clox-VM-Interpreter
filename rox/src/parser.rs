use crate::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    pub had_error: bool,
    pub in_panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Peekable<Iter<'a, Token>>) -> Parser<'a> {
        Parser {
            tokens,
            had_error: false,
            in_panic_mode: false,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.tokens.next() {
            println!("Parsed token: {token}");
        }
    }
}
