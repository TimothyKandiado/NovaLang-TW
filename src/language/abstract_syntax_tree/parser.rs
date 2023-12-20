
use crate::language::{
    errors,
    scanner::{
        object::Object,
        token::{Token, TokenType},
    },
};

use super::{
    expression::{
        binary::Binary, call::Call, grouping::Grouping, literal::Literal,
        unary::Unary, Expression, variable::Variable,
    },
    statement::{
        assignment::{Assign, Set, Get},
        declaration::VariableDeclaration,
        function::FunctionStatement,
        Block, IfStatement, Statement, WhileLoop,
    },
};

/// parses an abstract syntax tree from generated tokens
pub struct AstParser {
    tokens: Vec<Token>,
    current: usize,
}

const MAX_PARAMETERS: usize = 8;

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_ast(mut self) -> Result<Vec<Statement>, errors::Error> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.is_at_end() {
            let statement = self.declaration();
            if let Statement::None = statement {
                continue;
            }

            statements.push(statement)
        }
        return Ok(statements);
    }

    fn declaration(&mut self) -> Statement {
        let result = {
            if self.match_tokens(&[TokenType::Class]) {
                self.class_declaration()
            } else if self.match_tokens(&[TokenType::Fn]) {
                self.function_declaration("function")
            } else if self.match_tokens(&[TokenType::Let]) {
                self.var_declaration()
            } else {
                self.statement()
            }
        };

        if result.is_err() {
            self.synchronize();
            let err = result.unwrap_err();
            println!("{}", err);

            return Statement::None;
        }

        result.unwrap()
    }

    fn class_declaration(&mut self) -> Result<Statement, errors::Error> {
        todo!()
    }

    fn var_declaration(&mut self) -> Result<Statement, errors::Error> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name")?
            .clone();

        let mut initializer = None;

        if self.match_tokens(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::NewLine,
            "Expected new line after variable declaration",
        )?;

        Ok(Statement::VariableDeclaration(VariableDeclaration {
            name,
            initializer,
        }))
    }

    fn function_declaration(&mut self, kind: &str) -> Result<Statement, errors::Error> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name", kind))?;
        let name = name.clone();
        self.consume(TokenType::LeftParen, "Expect '(' before parameters")?;
        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() > MAX_PARAMETERS {
                    return Err(errors::Error::ParseError(format!(
                        "Cannot have more than {} parameters",
                        MAX_PARAMETERS
                    )));
                }

                let parameter = self.consume(TokenType::Identifier, "Expect parameter name")?;
                parameters.push(parameter.clone());

                if !self.match_tokens(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;
        self.consume(
            TokenType::NewLine,
            "Expect newline after function parameters",
        )?;
        let body = self.block_statement(&[TokenType::End], true)?;

        return Ok(Statement::FunctionStatement(Box::new(FunctionStatement {
            name,
            parameters,
            body,
        })));
    }

    fn statement(&mut self) -> Result<Statement, errors::Error> {
        if self.match_tokens(&[TokenType::For]) {
            return self.for_statement();
        }

        if self.match_tokens(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.match_tokens(&[TokenType::Return]) {
            return self.return_statement();
        }

        if self.match_tokens(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.match_tokens(&[TokenType::Block]) {
            return self.block_statement(&[TokenType::End], true);
        }

        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Statement, errors::Error> {
        todo!()
    }

    fn if_statement(&mut self) -> Result<Statement, errors::Error> {
        let condition = self.expression()?;

        let then_branch = self.block_statement(&[TokenType::End, TokenType::Else], false)?;
        let mut else_branch = None;

        if self.match_tokens(&[TokenType::Else]) {
            else_branch = Some(self.block_statement(&[TokenType::End], true)?)
        } else {
            self.consume(TokenType::End, "Expected end after if block")?;
            self.consume(TokenType::NewLine, "Expect new line after end")?;
        }

        return Ok(Statement::If(Box::new(IfStatement {
            condition,
            then_branch,
            else_branch,
        })));
    }

    fn return_statement(&mut self) -> Result<Statement, errors::Error> {
        let _keyword = self.previous();

        let mut value = None;

        if !self.check(TokenType::NewLine) {
            value = Some(self.expression()?);
        }

        return Ok(Statement::ReturnStatement(value));
    }

    fn while_statement(&mut self) -> Result<Statement, errors::Error> {
        //self.consume(TokenType::LeftParen, "Expect '(' before condition")?;
        let condition = self.expression()?;
        //self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        //self.consume(TokenType::NewLine, "Expect new line after while condition")?;

        let body = self.block_statement(&[TokenType::End], true)?;
        return Ok(Statement::WhileLoop(Box::new(WhileLoop {
            condition,
            body,
        })));
    }

    fn block_statement(
        &mut self,
        end_tokens: &[TokenType],
        consume: bool,
    ) -> Result<Statement, errors::Error> {
        let mut statements = Vec::new();
        self.consume(TokenType::NewLine, "Expect new line before block")?;

        while !self.match_tokens(end_tokens) && !self.is_at_end() {
            statements.push(self.declaration())
        }

        self.current -= 1;

        if consume {
            self.consume(TokenType::End, "Expect end of block")?;
            self.consume(TokenType::NewLine, "expect newline after block")?;
        }

        return Ok(Statement::Block(Block { statements }));
    }

    fn expression_statement(&mut self) -> Result<Statement, errors::Error> {
        let expression = self.expression()?;

        self.consume(TokenType::NewLine, "Expect newline after statement")?;
        Ok(Statement::ExpressionStatement(expression))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::NewLine {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => return,

                _ => {}
            }

            self.advance();
        }
    }

    pub fn expression(&mut self) -> Result<Expression, errors::Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, errors::Error> {
        let expression = self.or()?;

        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expression::Variable(variable) = &expression {
                let name = variable.name.clone();
                return Ok(Expression::Assign(Box::new(Assign { name, value })));
            } else if let Expression::Get(get) = &expression {
                let get = *get.clone();

                return Ok(Expression::Set(Box::new(Set {
                    name: get.name,
                    object: get.object,
                    value,
                })));
            }

            return Err(self.error(&equals, "Invalid Assignment Target"));
        }

        Ok(expression)
    }

    fn or(&mut self) -> Result<Expression, errors::Error> {
        let expression = self.and()?;

        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            let binary = Expression::Binary(Box::new(Binary {
                left: expression,
                right,
                operator,
            }));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn and(&mut self) -> Result<Expression, errors::Error> {
        let expression = self.equality()?;

        while self.match_tokens(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            let binary = Expression::Binary(Box::new(Binary {
                left: expression,
                right,
                operator,
            }));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression, errors::Error> {
        let expression = self.comparison()?;

        while self.match_tokens(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            let binary = Expression::Binary(Box::new(Binary {
                left: expression,
                right,
                operator,
            }));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, errors::Error> {
        let expression: Expression = self.addition()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition()?;

            let binary = Expression::Binary(Box::new(Binary {
                left: expression,
                right,
                operator,
            }));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn addition(&mut self) -> Result<Expression, errors::Error> {
        let mut expression = self.multiplication()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().to_owned();
            let right = self.multiplication()?;

            expression = Expression::Binary(Box::new(Binary::new(expression, right, operator)))
        }

        Ok(expression)
    }

    fn multiplication(&mut self) -> Result<Expression, errors::Error> {
        let mut expression = self.unary()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            expression = Expression::Binary(Box::new(Binary::new(expression, right, operator)))
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, errors::Error> {
        if self.match_tokens(&[TokenType::Minus]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            return Ok(Expression::Unary(Box::new(Unary::new(right, operator))));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expression, errors::Error> {
        let expression = self.primary()?;

        loop {
            if self.match_tokens(&[TokenType::LeftParen]) {
                return self.finish_call(expression)
            } 
            else if self.match_tokens(&[TokenType::Dot]) {
                let name = self.consume(TokenType::Identifier, "Expect name after '.'")?;
                return Ok(
                    Expression::Get(
                        Box::new(
                            Get {
                                object: expression,
                                name: name.clone()
                            }
                        )
                    )
                )
            }
            else {
                break;
            }
        }

        Ok(expression)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, errors::Error> {
            let mut arguments = Vec::new();
            if !self.check(TokenType::RightParen) {
                loop {
                    if arguments.len() > MAX_PARAMETERS {
                        return Err(self.error(self.previous(), "Too many arguments"));
                    }

                    arguments.push(self.expression()?);
                    if !self.match_tokens(&[TokenType::Comma]) {
                        break;
                    }
                }
            }

            let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments")?.clone();
            return Ok(
                Expression::Call(
                    Box::new(
                        Call::new(callee, paren, arguments)
                    )
                )
            )
    }

    fn primary(&mut self) -> Result<Expression, errors::Error> {
        // handle identifiers and function calls
        if self.match_tokens(&[TokenType::Identifier]) {
            let token = self.previous().clone();

            return Ok(Expression::Variable(Box::new(Variable::new(token))));
        }

        // Handle literals
        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let token = self.previous().clone();

            return Ok(Expression::Literal(Literal::new(token.object)));
        }

        // Handle booleans
        if self.match_tokens(&[TokenType::True, TokenType::False]) {
            let token = self.previous().clone();

            return Ok(Expression::Literal(Literal::new(Object::Bool(
                token.token_type == TokenType::True,
            ))));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expression::Grouping(Box::new(Grouping::new(expression))));
        }

        Err(self.error(self.peek(), "Expect Expression"))
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

    fn consume(
        &mut self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<&Token, errors::Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.previous(), error_message))
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

    /* fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    } */

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) -> errors::Error {
        errors::Error::ParseError(format!("[line: {}] (ParseError) {} ", token.line, message))
    }
}

#[cfg(test)]
mod test {
    
}
