use std::ops::Deref;
use std::rc::Rc;

use crate::RoxString;

#[derive(PartialEq, Debug)]
pub struct TokenStream(Vec<Token>);

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        let mut token_vec: Vec<Token> = vec![];

        for i in iter {
            token_vec.push(i);
        }

        TokenStream::new(token_vec)
    }
}

impl<'a> TokenStream {
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream(tokens)
    }

    pub fn iter(&self) -> std::slice::Iter<Token> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Token> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
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

impl std::cmp::PartialOrd for RoxNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::ops::Add<RoxNumber> for RoxNumber {
    type Output = Self;

    fn add(self, rhs: RoxNumber) -> Self::Output {
        RoxNumber(self.0 + rhs.0)
    }
}

impl std::ops::Neg for RoxNumber {
    type Output = Self;
    fn neg(self) -> Self::Output {
        RoxNumber(-self.0)
    }
}
impl std::ops::Sub<RoxNumber> for RoxNumber {
    type Output = Self;

    fn sub(self, rhs: RoxNumber) -> Self::Output {
        RoxNumber(self.0 - rhs.0)
    }
}

impl std::ops::Mul<RoxNumber> for RoxNumber {
    type Output = Self;

    fn mul(self, rhs: RoxNumber) -> Self::Output {
        RoxNumber(self.0 * rhs.0)
    }
}

impl std::ops::Div<RoxNumber> for RoxNumber {
    type Output = Self;

    fn div(self, rhs: RoxNumber) -> Self::Output {
        RoxNumber(self.0 / rhs.0)
    }
}

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

#[derive(Debug, Eq, Clone)]
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
    Identifier(Rc<RoxString>),
    StringLiteral(Rc<RoxString>),
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

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
