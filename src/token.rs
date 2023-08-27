/// A data type representing the tokens that come out of lexer
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    // Identifiers and literals
    Ident(String),
    Int(String),

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Assign,
    Bang,

    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,
    Hash,

    LParen,
    RParen,

    // Keywords
    Let,
    True,
    False,
    If,
    Then,
    Else,
    Return,
}
