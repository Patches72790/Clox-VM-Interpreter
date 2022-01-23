use crate::{
    token::{Token, TokenType},
    RoxNumber, DEBUG_MODE,
};
use std::{iter::Peekable, str::CharIndices};

type Peeker<'a> = Peekable<CharIndices<'a>>;

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    fn _is_at_end(line_chars: &mut Peeker) -> bool {
        match line_chars.peek() {
            Some(_) => false,
            None => true,
        }
    }

    fn check_next(
        line_chars: &mut Peeker,
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

    fn string(peeker: &mut Peeker) -> TokenType {
        let mut found_closing_quotation = false;
        let result: String = peeker
            .take_while(|(_, c)| {
                if *c == '"' {
                    found_closing_quotation = true;
                    return false;
                }
                true
            })
            .map(|(_, c)| c)
            .collect::<String>();

        if !found_closing_quotation {
            return TokenType::Error(String::from("Unterminated string literal"));
        }
        TokenType::StringLiteral(result)
    }

    fn number(peeker: &mut Peeker, ch: &char) -> TokenType {
        let mut string_of_num = ch.to_string();
        while let Some((_, c)) = peeker.next_if(|(_, c)| c.is_numeric() || *c == '.') {
            string_of_num.push(c)
        }

        match string_of_num.parse::<f32>() {
            Ok(val) => TokenType::Number(RoxNumber(val)),
            Err(_) => TokenType::Error(format!("Error parsing number {}", string_of_num)),
        }
    }

    fn identifier(peeker: &mut Peeker, first_letter: &char) -> TokenType {
        todo!("Need to finish identifiers and keywords!")
    }

    pub fn scan_tokens(&self, source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let mut line_chars: Peeker = line.char_indices().peekable();
            while let Some((char_num, ch)) = line_chars.next() {
                let token_type = match ch {
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ',' => TokenType::Comma,
                    '.' => {
                        if line_chars.peek().unwrap_or(&(0, ' ')).1.is_numeric() {
                            while let Some((_, _)) = line_chars.next_if(|(_, c)| c.is_numeric()) {}
                            TokenType::Error(String::from(
                                "Cannot begin a number in Rox with a dot.",
                            ))
                        } else {
                            TokenType::Dot
                        }
                    }
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
                    '"' => Scanner::string(&mut line_chars),
                    '0'..='9' => Scanner::number(&mut line_chars, &ch),
                    'a'..='z' | 'A'..='Z' => Scanner::identifier(&mut line_chars, &ch),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_ops() {
        let scanner = Scanner::new();
        let source = String::from("() {} , ;");

        let tokens = scanner.scan_tokens(&source);

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::LeftParen, 1, 1),
                Token::new(TokenType::RightParen, 1, 2),
                Token::new(TokenType::LeftBrace, 1, 4),
                Token::new(TokenType::RightBrace, 1, 5),
                Token::new(TokenType::Comma, 1, 7),
                Token::new(TokenType::Semicolon, 1, 9),
            ]
        );
    }

    #[test]
    fn test_string_literal() {}

    #[test]
    fn test_number_literal() {}

    #[test]
    fn test_identifier() {}

    #[test]
    fn test_keywords() {}

    #[test]
    fn test_error_tokens() {}
}
