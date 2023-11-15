use super::object::Object;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Star,
    Divide,
    LeftParen,
    RightParen,
    Identifier,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub object: Object,
}
