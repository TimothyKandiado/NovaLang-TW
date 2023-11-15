use crate::interpreter::scanner::token::{Token, TokenType};

use super::expression::{
    binary::Binary, grouping::Grouping, literal::Literal, math_function::MathFunction,
    unary::Unary, Expression,
};

pub struct AstParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Result<Expression, String> {
        self.addition()
    }

    fn addition(&mut self) -> Result<Expression, String> {
        let mut expression = self.multiplication()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().to_owned();
            let right = self.multiplication()?;

            expression = Expression::Binary(Box::new(Binary::new(expression, right, operator)))
        }

        Ok(expression)
    }

    fn multiplication(&mut self) -> Result<Expression, String> {
        let mut expression = self.unary()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Divide]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            expression = Expression::Binary(Box::new(Binary::new(expression, right, operator)))
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[TokenType::Minus]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            return Ok(Expression::Unary(Box::new(Unary::new(right, operator))));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[TokenType::Number, TokenType::Identifier]) {
            let token = self.previous().clone();
            if self.match_tokens(&[TokenType::LeftParen]) {
                let function_id = token.object;
                let arguments = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after function call")?;

                return Ok(Expression::MathFunction(Box::new(MathFunction::new(
                    function_id,
                    arguments,
                ))));
            }

            return Ok(Expression::Literal(Literal::new(token.object)));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expression::Grouping(Box::new(Grouping::new(expression))));
        }

        Err("Expect Expression".to_string())
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for &token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(error_message.to_string())
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
