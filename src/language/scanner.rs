pub mod object;
pub mod token;

use crate::language::errors;
use token::{Token, TokenType};

fn simple_token(token_type: TokenType, line: usize) -> Token {
    Token {
        token_type,
        object: object::Object::None,
        line,
    }
}

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,
    line: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self, source: &str) -> Result<Vec<Token>, errors::Error> {
        self.source = source.to_string();
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.scan_token()?;
            tokens.push(token);
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            object: object::Object::None,
            line: self.line,
        });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, errors::Error> {
        let newline_token = self.skip_whitespace();
        if let Some(newline) = newline_token {
            return Ok(newline);
        }

        self.start = self.current;
        let current_character = self.advance();

        match current_character {
            '+' => Ok(simple_token(TokenType::Plus, self.line)),
            '-' => Ok(simple_token(TokenType::Minus, self.line)),
            '*' => Ok(simple_token(TokenType::Star, self.line)),
            '/' => Ok(simple_token(TokenType::Slash, self.line)),
            '^' => Ok(simple_token(TokenType::Caret, self.line)),
            '%' => Ok(simple_token(TokenType::Percent, self.line)),

            '(' => Ok(simple_token(TokenType::LeftParen, self.line)),
            ')' => Ok(simple_token(TokenType::RightParen, self.line)),
            ':' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::ColonEqual, self.line));
                }

                Ok(simple_token(TokenType::Colon, self.line))
            }
            '.' => Ok(simple_token(TokenType::Dot, self.line)),
            ',' => Ok(simple_token(TokenType::Comma, self.line)),
            '"' => self.scan_string(),
            '&' => {
                let next = self.advance();
                if next == '&' {
                    return Ok(simple_token(TokenType::And, self.line));
                }

                Err(errors::Error::Scan(format!(
                    "Unknown token {}",
                    current_character
                )))
            }

            '|' => {
                let next = self.advance();
                if next != '|' {
                    return Err(errors::Error::Scan("Unknown token '|' ".to_string()));
                }
                Ok(simple_token(TokenType::Or, self.line))
            }

            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::GreaterEqual, self.line));
                }
                Ok(simple_token(TokenType::Greater, self.line))
            }

            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::LessEqual, self.line));
                }
                Ok(simple_token(TokenType::Less, self.line))
            }

            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::EqualEqual, self.line));
                }
                Ok(simple_token(TokenType::Equal, self.line))
            }

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::NotEqual, self.line));
                }

                Ok(simple_token(TokenType::Not, self.line))
            }

            x if x.is_ascii_digit() => self.scan_number(),
            x if is_identifier_start(x) => self.scan_identifier(),

            _ => Err(errors::Error::Scan(format!(
                "Undefined character {}",
                current_character
            ))),
        }
    }

    fn skip_whitespace(&mut self) -> Option<Token> {
        let mut has_consumed_newline = false;

        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' => {
                    self.advance();
                }

                '\n' => {
                    has_consumed_newline = true;
                    self.line += 1;
                    self.advance();
                }

                '#' => {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    self.advance();
                }

                _ => break,
            }
        }

        if has_consumed_newline {
            Some(Token {
                token_type: TokenType::NewLine,
                object: object::Object::None,
                line: self.line,
            })
        } else {
            None
        }
    }

    fn scan_number(&mut self) -> Result<Token, errors::Error> {
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
                return Err(errors::Error::Scan(
                    "could not parse exponent value".to_string(),
                ));
            }
        }

        if number_result.is_err() {
            return Err(errors::Error::Scan(format!(
                "could not parse float from {}",
                segment
            )));
        }

        if let Ok(number) = number_result {
            let number = number * 10f64.powf(exponent * exponent_sign as f64);
            Ok(Token {
                token_type: TokenType::Number,
                object: object::Object::Number(number),
                line: self.line,
            })
        } else {
            Err(errors::Error::Scan(format!(
                "could not parse number from {}",
                segment
            )))
        }
    }

    fn scan_identifier(&mut self) -> Result<Token, errors::Error> {
        while !self.is_at_end() && is_identifier_rest(self.peek()) {
            self.advance();
        }

        let segment = &self.source[self.start..self.current];

        match segment {
            "for" => Ok(simple_token(TokenType::For, self.line)),
            "if" => Ok(simple_token(TokenType::If, self.line)),
            "else" => Ok(simple_token(TokenType::Else, self.line)),
            "while" => Ok(simple_token(TokenType::While, self.line)),
            "fn" => Ok(simple_token(TokenType::Fn, self.line)),
            "end" => Ok(simple_token(TokenType::End, self.line)),
            "return" => Ok(simple_token(TokenType::Return, self.line)),
            "true" => Ok(simple_token(TokenType::True, self.line)),
            "false" => Ok(simple_token(TokenType::False, self.line)),
            "and" => Ok(simple_token(TokenType::And, self.line)),
            "or" => Ok(simple_token(TokenType::Or, self.line)),
            "class" => Ok(simple_token(TokenType::Class, self.line)),
            "let" => Ok(simple_token(TokenType::Let, self.line)),
            "block" => Ok(simple_token(TokenType::Block, self.line)),
            "delete" => Ok(simple_token(TokenType::Delete, self.line)),
            "none" => Ok(simple_token(TokenType::None, self.line)),
            "include" => Ok(simple_token(TokenType::Include, self.line)),

            _ => Ok(Token {
                token_type: TokenType::Identifier,
                object: object::Object::String(segment.to_string()),
                line: self.line,
            }),
        }
    }

    fn scan_string(&mut self) -> Result<Token, errors::Error> {
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }
        self.consume('"', "Expect \" at end of string")?;

        let mut string = self.source[self.start..self.current].to_string();
        string.remove(0);
        string.remove(string.len() - 1);

        Ok(Token {
            token_type: TokenType::String,
            object: object::Object::String(string),
            line: self.line,
        })
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

    fn consume(&mut self, character: char, message: &str) -> Result<(), errors::Error> {
        if self.peek() == character {
            self.advance();
            return Ok(());
        }

        Err(errors::Error::Scan(message.to_string()))
    }
}

fn is_identifier_start(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

fn is_identifier_rest(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
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
                    object: Object::Number(100.0),
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None,
                    line: 1
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
                    object: Object::String("sin".to_string()),
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None,
                    line: 1
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
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::While,
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::NewLine,
                    object: Object::None,
                    line: 2
                },
                Token {
                    token_type: TokenType::Fn,
                    object: Object::None,
                    line: 2
                },
                Token {
                    token_type: TokenType::End,
                    object: Object::None,
                    line: 2
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None,
                    line: 2
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
                    object: Object::Number(1.0),
                    line: 1
                },
                Token {
                    token_type: TokenType::Plus,
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(2.0),
                    line: 1
                },
                Token {
                    token_type: TokenType::Slash,
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::LeftParen,
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(3.0),
                    line: 1
                },
                Token {
                    token_type: TokenType::Plus,
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::Number,
                    object: Object::Number(1.0),
                    line: 1
                },
                Token {
                    token_type: TokenType::RightParen,
                    object: Object::None,
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None,
                    line: 1
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
                simple_token(TokenType::EqualEqual, 1),
                simple_token(TokenType::GreaterEqual, 1),
                simple_token(TokenType::LessEqual, 1),
                simple_token(TokenType::Greater, 1),
                simple_token(TokenType::Less, 1),
                simple_token(TokenType::NotEqual, 1),
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None,
                    line: 1
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
                simple_token(TokenType::And, 1),
                simple_token(TokenType::Or, 1),
                simple_token(TokenType::Not, 1),
                simple_token(TokenType::And, 1),
                simple_token(TokenType::Or, 1),
                Token {
                    token_type: TokenType::Eof,
                    object: Object::None,
                    line: 1
                }
            ]
        )
    }
}
