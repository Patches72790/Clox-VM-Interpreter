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
        let mut string_accum = first_letter.to_string();
        while let Some((_, c)) = peeker.next_if(|(_, c)| c.is_ascii_alphanumeric() || *c == '_') {
            string_accum.push(c);
        }

        Scanner::find_identifier_type(&string_accum)
    }

    fn find_identifier_type(id: &str) -> TokenType {
        let mut id_chars = id.char_indices().peekable();
        match id_chars.next().unwrap_or((0, '!')) {
            (.., 'a') => Scanner::check_keyword(&mut id_chars, 2, "nd", id, TokenType::And),
            (.., 'c') => Scanner::check_keyword(&mut id_chars, 4, "lass", id, TokenType::Class),
            (.., 'e') => Scanner::check_keyword(&mut id_chars, 3, "lse", id, TokenType::Else),
            (.., 'i') => Scanner::check_keyword(&mut id_chars, 1, "f", id, TokenType::If),
            (.., 'n') => Scanner::check_keyword(&mut id_chars, 2, "il", id, TokenType::Nil),
            (.., 'o') => Scanner::check_keyword(&mut id_chars, 1, "r", id, TokenType::Or),
            (.., 'p') => Scanner::check_keyword(&mut id_chars, 4, "rint", id, TokenType::Print),
            (.., 'r') => Scanner::check_keyword(&mut id_chars, 5, "eturn", id, TokenType::Return),
            (.., 's') => Scanner::check_keyword(&mut id_chars, 4, "uper", id, TokenType::Super),
            (.., 'v') => Scanner::check_keyword(&mut id_chars, 2, "ar", id, TokenType::Var),
            (.., 'w') => Scanner::check_keyword(&mut id_chars, 4, "hile", id, TokenType::While),
            (.., 'f') => match id_chars.next().unwrap_or((0, '!')) {
                (.., 'a') => Scanner::check_keyword(&mut id_chars, 3, "lse", id, TokenType::False),
                (.., 'o') => Scanner::check_keyword(&mut id_chars, 1, "r", id, TokenType::For),
                (.., 'u') => Scanner::check_keyword(&mut id_chars, 1, "n", id, TokenType::Fun),
                (.., '!') => TokenType::Error(
                    "Error grabbing next char after 'f' in scanning identifier.".to_string(),
                ),
                _ => TokenType::Identifier(id.to_string()),
            },
            (.., 't') => match id_chars.next().unwrap_or((0, '!')) {
                (.., 'h') => Scanner::check_keyword(&mut id_chars, 2, "is", id, TokenType::This),
                (.., 'r') => Scanner::check_keyword(&mut id_chars, 2, "ue", id, TokenType::True),
                (.., '!') => TokenType::Error(
                    "Error grabbing next char after 't' in scanning identifier".to_string(),
                ),
                _ => TokenType::Identifier(id.to_string()),
            },
            (.., '!') => {
                TokenType::Error("Error grabbing first char in scanning identifer".to_string())
            }
            _ => TokenType::Identifier(id.to_string()),
        }
    }

    fn check_keyword(
        char_peeker: &mut Peeker,
        length: usize,
        suffix: &str,
        original_str: &str,
        t_type: TokenType,
    ) -> TokenType {
        let result = char_peeker
            .take(length)
            .map(|(_, c)| c.to_string())
            .collect::<String>();

        println!("Result: {}", result);
        if result == suffix && char_peeker.next() == None {
            t_type
        } else {
            TokenType::Identifier(original_str.to_string())
        }
    }

    pub fn scan_tokens(&self, source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut num_lines = 1;

        for (line_num, line) in source.lines().enumerate() {
            let mut line_chars: Peeker = line.char_indices().peekable();
            while let Some((char_num, ch)) = line_chars.next() {
                let token_type = match ch {
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ',' => TokenType::Comma,
                    ';' => TokenType::Semicolon,
                    '.' => {
                        if line_chars.peek().unwrap_or(&(0, ' ')).1.is_numeric() {
                            while let Some(_) = line_chars.next_if(|(_, c)| c.is_numeric()) {}
                            TokenType::Error(String::from(
                                "Cannot begin a number in Rox with a dot.",
                            ))
                        } else {
                            TokenType::Dot
                        }
                    }
                    '-' => TokenType::Minus,
                    '+' => TokenType::Plus,
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
            num_lines += 1;
        }

        // add token EOF sentinel for signaling end of scanner token stream
        tokens.push(Token::new(TokenType::EOF, num_lines, 1));

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
