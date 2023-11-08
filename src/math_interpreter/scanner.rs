pub mod object;
pub mod token;

use token::{Token, TokenType};

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(mut self, source: &str) -> Result<Vec<Token>, String> {
        self.source = source.to_string();
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.scan_token()?;
            tokens.push(token);
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            object: object::Object::None,
        });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        self.start = self.current;
        let current_character = self.advance();

        match current_character {
            '+' => Ok(Token {
                token_type: TokenType::Plus,
                object: object::Object::None,
            }),
            '-' => Ok(Token {
                token_type: TokenType::Minus,
                object: object::Object::None,
            }),
            '*' => Ok(Token {
                token_type: TokenType::Star,
                object: object::Object::None,
            }),
            '/' => Ok(Token {
                token_type: TokenType::Divide,
                object: object::Object::None,
            }),

            '(' => Ok(Token {
                token_type: TokenType::LeftParen,
                object: object::Object::None,
            }),
            ')' => Ok(Token {
                token_type: TokenType::RightParen,
                object: object::Object::None,
            }),

            x if x.is_digit(10) => self.scan_number(),
            x if x.is_alphabetic() => self.scan_string(),

            _ => Err(format!("Undefined character {}", current_character)),
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\n' | '\r' => {
                    self.advance();
                }

                _ => break,
            }
        }
    }

    fn scan_number(&mut self) -> Result<Token, String> {
        // consume all digits until the end or non digit character
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        let next = self.peek();
        // if next character is a decimal point consume all remaining digits
        if next == '.' {
            self.advance();
            while !self.is_at_end() && self.peek().is_digit(10) {
                self.advance();
            }
        }

        let segment = &self.source[self.start..self.current].to_string();
        let number_result = segment.parse::<f64>();

        let next = self.peek();
        let mut exponent = 0f64;
        let mut exponent_sign = 1; // sign of exponent
                                   // scan exponent section if any
        if next == 'E' {
            self.advance();

            if self.peek() == '-' {
                exponent_sign = -1;
                self.advance();
            }
            self.start = self.current;

            while !self.is_at_end() && self.peek().is_digit(10) {
                self.advance();
            }
            let exponent_segment = &self.source[self.start..self.current].to_string();
            if let Ok(exponent_value) = exponent_segment.parse::<f64>() {
                exponent = exponent_value;
            } else {
                return Err(format!("could not parse exponent value"));
            }
        }

        if number_result.is_err() {
            return Err(format!("could not parse float from {}", segment));
        }

        if let Ok(number) = number_result {
            let number = number * 10f64.powf(exponent * exponent_sign as f64);
            Ok(Token {
                token_type: TokenType::Number,
                object: object::Object::Number(number),
            })
        } else {
            Err(format!("could not parse number from {}", segment))
        }
    }

    fn scan_string(&mut self) -> Result<Token, String> {
        while !self.is_at_end() && self.peek().is_alphabetic() {
            self.advance();
        }

        let segment = &self.source[self.start..self.current];

        Ok(Token {
            token_type: TokenType::Identifier,
            object: object::Object::String(segment.to_string()),
        })
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    /* fn peek_next(&self) -> char {
        if self.is_at_end() || self.current + 1 >= self.source.len() {
            return '\0'
        }

        self.source.chars().nth(self.current + 1).unwrap()
    } */

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
