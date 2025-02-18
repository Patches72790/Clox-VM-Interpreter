use crate::{
    token::{Token, TokenType},
    RoxNumber, RoxString, TokenStream, DEBUG_MODE,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::{iter::Peekable, str::CharIndices};

type Peeker<'a> = Peekable<CharIndices<'a>>;

#[derive(Default)]
pub struct Scanner {
    had_error: RefCell<bool>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            had_error: RefCell::new(false),
        }
    }

    fn _is_at_end(line_chars: &mut Peeker) -> bool {
        line_chars.peek().is_none()
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

        TokenType::StringLiteral(Rc::new(RoxString::new(&result)))
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
            (.., 'b') => Scanner::check_keyword(&mut id_chars, 4, "reak", id, TokenType::Break),
            (.., 'c') => match id_chars.next().unwrap_or((0, '!')) {
                (.., 'a') => Scanner::check_keyword(&mut id_chars, 2, "se", id, TokenType::Case),
                (.., 'l') => Scanner::check_keyword(&mut id_chars, 3, "ass", id, TokenType::Class),
                (.., 'o') => {
                    Scanner::check_keyword(&mut id_chars, 6, "ntinue", id, TokenType::Continue)
                }
                (.., '!') => TokenType::Error(
                    "Error grabbing next char after 'c' in scanning identifier.".to_string(),
                ),
                _ => TokenType::Identifier(Rc::new(RoxString::new(id))),
            },
            (.., 'd') => Scanner::check_keyword(&mut id_chars, 6, "efault", id, TokenType::Default),
            (.., 'e') => Scanner::check_keyword(&mut id_chars, 3, "lse", id, TokenType::Else),
            (.., 'i') => Scanner::check_keyword(&mut id_chars, 1, "f", id, TokenType::If),
            (.., 'n') => Scanner::check_keyword(&mut id_chars, 2, "il", id, TokenType::Nil),
            (.., 'o') => Scanner::check_keyword(&mut id_chars, 1, "r", id, TokenType::Or),
            (.., 'p') => Scanner::check_keyword(&mut id_chars, 4, "rint", id, TokenType::Print),
            (.., 'r') => Scanner::check_keyword(&mut id_chars, 5, "eturn", id, TokenType::Return),
            (.., 's') => match id_chars.next().unwrap_or((0, '!')) {
                (.., 'u') => Scanner::check_keyword(&mut id_chars, 3, "per", id, TokenType::Super),
                (.., 'w') => {
                    Scanner::check_keyword(&mut id_chars, 4, "itch", id, TokenType::Switch)
                }
                (.., '!') => TokenType::Error(
                    "Error grabbing next char after 's' in scanning identifier.".to_string(),
                ),
                _ => TokenType::Identifier(Rc::new(RoxString::new(id))),
            },
            (.., 'v') => Scanner::check_keyword(&mut id_chars, 2, "ar", id, TokenType::Var),
            (.., 'w') => Scanner::check_keyword(&mut id_chars, 4, "hile", id, TokenType::While),
            (.., 'f') => match id_chars.next().unwrap_or((0, '!')) {
                (.., 'a') => Scanner::check_keyword(&mut id_chars, 3, "lse", id, TokenType::False),
                (.., 'o') => Scanner::check_keyword(&mut id_chars, 1, "r", id, TokenType::For),
                (.., 'u') => Scanner::check_keyword(&mut id_chars, 1, "n", id, TokenType::Fun),
                (.., '!') => TokenType::Error(
                    "Error grabbing next char after 'f' in scanning identifier.".to_string(),
                ),
                _ => TokenType::Identifier(Rc::new(RoxString::new(id))),
            },
            (.., 't') => match id_chars.next().unwrap_or((0, '!')) {
                (.., 'h') => Scanner::check_keyword(&mut id_chars, 2, "is", id, TokenType::This),
                (.., 'r') => Scanner::check_keyword(&mut id_chars, 2, "ue", id, TokenType::True),
                (.., '!') => TokenType::Error(
                    "Error grabbing next char after 't' in scanning identifier".to_string(),
                ),
                _ => TokenType::Identifier(Rc::new(RoxString::new(id))),
            },
            (.., '!') => {
                TokenType::Error("Error grabbing first char in scanning identifer".to_string())
            }
            _ => TokenType::Identifier(Rc::new(RoxString::new(id))),
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

        if result == suffix && char_peeker.next().is_none() {
            t_type
        } else {
            TokenType::Identifier(Rc::new(RoxString::new(original_str)))
        }
    }

    pub fn scan_tokens(&self, source: &str) -> TokenStream {
        let mut tokens: Vec<Token> = Vec::new();
        let mut num_lines = 1;

        for (line_num, line) in source.lines().enumerate() {
            let mut line_chars: Peeker = line.char_indices().peekable();
            while let Some((char_num, ch)) = line_chars.next() {
                let token_type = match ch {
                    ':' => TokenType::Colon,
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ',' => TokenType::Comma,
                    ';' => TokenType::Semicolon,
                    '.' => {
                        if line_chars.peek().unwrap_or(&(0, ' ')).1.is_numeric() {
                            while line_chars.next_if(|(_, c)| c.is_numeric()).is_some() {}
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
                            for (_, c) in line_chars.by_ref() {
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

                if let TokenType::Error(_) = token_type {
                    *self.had_error.borrow_mut() = true
                }

                tokens.push(self.scan_token(token_type, line_num + 1, char_num + 1));
            }
            num_lines += 1;
        }

        // add token EOF sentinel for signaling end of scanner token stream
        tokens.push(Token::new(TokenType::EOF, num_lines, 1));

        if DEBUG_MODE {
            tokens
                .iter()
                .for_each(|token| println!("Scanned Token: {}", token));
        }
        TokenStream::new(tokens)
    }

    pub fn had_error(&self) -> bool {
        *self.had_error.borrow()
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
            *tokens,
            vec![
                Token::new(TokenType::LeftParen, 1, 1),
                Token::new(TokenType::RightParen, 1, 2),
                Token::new(TokenType::LeftBrace, 1, 4),
                Token::new(TokenType::RightBrace, 1, 5),
                Token::new(TokenType::Comma, 1, 7),
                Token::new(TokenType::Semicolon, 1, 9),
                Token::new(TokenType::EOF, 2, 1),
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
