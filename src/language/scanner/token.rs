use super::object::Object;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Colon,
    LeftParen,
    RightParen,
    Identifier,
    Eof,

    Fn,
    For,
    While,
    NewLine,
    End,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub object: Object,
}
