use crate::location;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    And,
    Bang,
    BangEqual,
    Class,
    Comma,
    Dot,
    Else,
    Eof,
    Equal,
    EqualEqual,
    Error,
    False,
    For,
    Fun,
    Greater,
    GreaterEqual,
    Identifier,
    If,
    LeftBrace,
    LeftParen,
    Less,
    LessEqual,
    Minus,
    Nil,
    Number,
    Or,
    Plus,
    Print,
    Return,
    RightParen,
    RightBrace,
    Slash,
    Semicolon,
    Star,
    String,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub location: location::Region,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, location: location::Region) -> Self {
        Self {
            token_type,
            lexeme,
            location,
        }
    }
}
