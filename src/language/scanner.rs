pub mod object;
pub mod token;

use token::{Token, TokenType};

fn simple_token(token_type: TokenType) -> Token {
    Token {
        token_type,
        object: object::Object::None,
    }
}

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
        let newline_token = self.skip_whitespace();
        if let Some(newline) = newline_token {
            return Ok(newline);
        }

        self.start = self.current;
        let current_character = self.advance();

        match current_character {
            '+' => Ok(simple_token(TokenType::Plus)),
            '-' => Ok(simple_token(TokenType::Minus)),
            '*' => Ok(simple_token(TokenType::Star)),
            '/' => Ok(simple_token(TokenType::Slash)),

            '(' => Ok(simple_token(TokenType::LeftParen)),
            ')' => Ok(simple_token(TokenType::RightParen)),
            ':' => Ok(simple_token(TokenType::Colon)),
            '&' => {
                let next = self.advance();
                if next == '&' {
                    return Ok(simple_token(TokenType::And));
                }

                Err(format!("Unknown token {}", current_character))
            }

            '|' => {
                let next = self.advance();
                if next != '|' {
                    return Err(format!("Unknown token '|' "));
                }
                Ok(simple_token(TokenType::Or))
            }

            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::GreaterEqual));
                }
                Ok(simple_token(TokenType::Greater))
            }

            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::LessEqual));
                }
                Ok(simple_token(TokenType::Less))
            }

            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::EqualEqual));
                }
                Ok(simple_token(TokenType::Assign))
            }

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::NotEqual));
                }

                Ok(simple_token(TokenType::Not))
            }

            x if x.is_ascii_digit() => self.scan_number(),
            x if x.is_alphabetic() => self.scan_identifier(),

            _ => Err(format!("Undefined character {}", current_character)),
        }
    }

    fn skip_whitespace(&mut self) -> Option<Token> {
        let mut has_consumed_newline = false;

        while !self.is_at_end() {
            if !has_consumed_newline && self.peek() == '\n' {
                has_consumed_newline = true;
            }
            match self.peek() {
                ' ' | '\n' | '\r' => {
                    self.advance();
                }

                _ => break,
            }
        }

        if has_consumed_newline {
            Some(Token {
                token_type: TokenType::NewLine,
                object: object::Object::None,
            })
        } else {
            None
        }
    }

    fn scan_number(&mut self) -> Result<Token, String> {
        // consume all digits until the end or non digit character
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        let next = self.peek();
        // if next character is a decimal point consume all remaining digits
        if next == '.' {
            self.advance();
            while !self.is_at_end() && self.peek().is_ascii_digit() {
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

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
            let exponent_segment = &self.source[self.start..self.current].to_string();
            if let Ok(exponent_value) = exponent_segment.parse::<f64>() {
                exponent = exponent_value;
            } else {
                return Err("could not parse exponent value".to_string());
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

    fn scan_identifier(&mut self) -> Result<Token, String> {
        while !self.is_at_end() && self.peek().is_alphabetic() {
            self.advance();
        }

        let segment = &self.source[self.start..self.current];

        match segment {
            "for" => Ok(simple_token(TokenType::For)),
            "while" => Ok(simple_token(TokenType::While)),
            "fn" => Ok(simple_token(TokenType::Fn)),
            "end" => Ok(simple_token(TokenType::End)),
            "return" => Ok(simple_token(TokenType::Return)),
            "true" => Ok(simple_token(TokenType::True)),
            "false" => Ok(simple_token(TokenType::False)),
            "and" => Ok(simple_token(TokenType::And)),
            "or" => Ok(simple_token(TokenType::Or)),

            _ => Ok(Token {
                token_type: TokenType::Identifier,
                object: object::Object::String(segment.to_string()),
            }),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

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

#[cfg(test)]
mod tests {
    use crate::language::scanner::object::Object;
    use crate::language::scanner::token::Token;
    use crate::language::scanner::token::TokenType;

    use super::{simple_token, Scanner};

    #[test]
    fn test_scanner_number() {
        let source = "100";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(100.0)
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None
                }
            ]
        )
    }

    #[test]
    fn test_scanner_identifier() {
        let source = "sin";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Identifier,
                    object: Object::String("sin".to_string())
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None
                }
            ]
        )
    }

    #[test]
    fn test_scanner_keywords() {
        let source = "for while \n fn end";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::For,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::While,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::NewLine,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::Fn,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::End,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None
                }
            ]
        )
    }

    #[test]
    fn test_scanner_simple_expression() {
        let source = "1 + 2 / ( 3 + 1 )";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(1.0)
                },
                Token {
                    token_type: TokenType::Plus,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(2.0)
                },
                Token {
                    token_type: TokenType::Slash,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::LeftParen,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(3.0)
                },
                Token {
                    token_type: TokenType::Plus,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(1.0)
                },
                Token {
                    token_type: TokenType::RightParen,
                    object: Object::None
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None
                }
            ]
        )
    }

    #[test]
    fn test_scanner_comparison_operators() {
        let source = "== >= <= > < !=";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                simple_token(TokenType::EqualEqual),
                simple_token(TokenType::GreaterEqual),
                simple_token(TokenType::LessEqual),
                simple_token(TokenType::Greater),
                simple_token(TokenType::Less),
                simple_token(TokenType::NotEqual),
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None
                }
            ]
        )
    }

    #[test]
    fn test_scanner_logical_operators() {
        let source = "&& || ! and or";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                simple_token(TokenType::And),
                simple_token(TokenType::Or),
                simple_token(TokenType::Not),
                simple_token(TokenType::And),
                simple_token(TokenType::Or),
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None
                }
            ]
        )
    }
}
