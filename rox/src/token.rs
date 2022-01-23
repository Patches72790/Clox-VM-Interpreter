use std::ops::Deref;

#[derive(Debug, Clone, Eq)]
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

impl std::cmp::PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RoxNumber(pub f32);

impl std::cmp::Eq for RoxNumber {}

impl std::fmt::Display for RoxNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for RoxNumber {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
