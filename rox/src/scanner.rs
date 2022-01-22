use crate::{
    token::{Token, TokenType},
    DEBUG_MODE,
};
use std::{iter::Peekable, rc::Rc, str::CharIndices};

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    fn is_at_end(line_chars: &mut Peekable<CharIndices>) -> bool {
        match line_chars.peek() {
            Some(_) => false,
            None => true,
        }
    }

    fn check_next(
        line_chars: &mut Peekable<CharIndices>,
        check: char,
        first: TokenType,
        second: TokenType,
    ) -> TokenType {
        let mut _t_type = first;
        if line_chars.peek().unwrap_or(&(0, ' ')).1 == check {
            line_chars.next();
            _t_type = second;
        }
        _t_type
    }

    fn string() -> TokenType {
        todo!("Need to finish string literals!")
    }
    fn number() -> TokenType {
        todo!("Need to finish number literals!")
    }

    fn identifier() -> TokenType {
        todo!("Need to finish identifiers and keywords!")
    }

    pub fn scan_tokens(&self, source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let mut line_chars = line.char_indices().peekable();
            while let Some((char_num, ch)) = line_chars.next() {
                let token_type = match ch {
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '-' => TokenType::Minus,
                    '+' => TokenType::Plus,
                    ';' => TokenType::Semicolon,
                    '*' => TokenType::Star,
                    '!' => Scanner::check_next(
                        &mut line_chars,
                        '=',
                        TokenType::Bang,
                        TokenType::BangEqual,
                    ),
                    '=' => Scanner::check_next(
                        &mut line_chars,
                        '=',
                        TokenType::Equal,
                        TokenType::EqualEqual,
                    ),
                    '>' => Scanner::check_next(
                        &mut line_chars,
                        '=',
                        TokenType::Greater,
                        TokenType::GreaterEqual,
                    ),
                    '<' => Scanner::check_next(
                        &mut line_chars,
                        '=',
                        TokenType::Less,
                        TokenType::LessEqual,
                    ),
                    ' ' | '\n' | '\t' | '\r' => continue, // skip whitespace
                    '/' => {
                        if line_chars.peek().unwrap_or(&(0, ' ')).1 == '/' {
                            while let Some((_, c)) = line_chars.next() {
                                match c {
                                    '\n' => break,
                                    _ => continue,
                                }
                            }
                            continue;
                        } else {
                            TokenType::Slash
                        }
                    }
                    '"' => Scanner::string(),
                    '0'..='9' => Scanner::number(),
                    'a'..='z' | 'A'..='Z' => Scanner::identifier(),
                    _ => TokenType::Error(String::from("Unexpected char read from source")),
                };

                tokens.push(self.scan_token(token_type, line_num + 1, char_num + 1));
            }
        }
        if DEBUG_MODE {
            tokens.iter().for_each(|token| println!("Token: {}", token));
        }
        tokens
    }

    fn scan_token(&self, token_type: TokenType, line: usize, column: usize) -> Token {
        Token::new(token_type, line, column)
    }
}
