use std::fmt::Display;

use super::object::Object;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenType {
    Number,
    String,
    Plus,
    Minus,
    Star,
    Slash,
    Colon,
    LeftParen,
    RightParen,
    Identifier,
    Dot,
    Comma,
    Eof,

    Fn,
    Class,
    Let,
    Block,
    If,
    Else,
    For,
    While,
    NewLine,
    End,
    Return,
    //Assign,

    True,
    False,
    And,
    Or,
    Not,
    Equal,
    EqualEqual,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub object: Object,
    pub line: usize
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.token_type {
            TokenType::Number | TokenType::String | TokenType::Identifier => {
                format!("({:?} : {})",self.token_type, self.object.to_string())
            }

            _ => {format!("{:?}", self.token_type)}
        })
    }
}

pub fn debug_print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        print!("{} ", &token);
        if TokenType::NewLine == token.token_type {
            println!("");
        }
    }

    println!("");
}