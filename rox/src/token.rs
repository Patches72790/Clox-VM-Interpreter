use std::ops::Deref;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let my_type = &self.token_type;
        let line = &self.line;
        let col = &self.column;
        write!(
            f,
            "<Token lexeme: type: {my_type:?}, line: {line}, col: {col}>"
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Token {
        Token {
            token_type,
            line,
            column,
        }
    }
}

#[derive(Debug)]
pub struct RoxNumber(f32);

impl Deref for RoxNumber {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier(String),
    StringLiteral(String),
    Number(RoxNumber),
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error(String),
    EOF,
}
