use crate::language::{
    class::ClassStatement,
    errors,
    scanner::{
        object::Object,
        token::{Token, TokenType}, TokenContainer,
    }, Include,
};

use super::{
    expression::{
        binary::Binary, call::Call, grouping::Grouping, literal::Literal, unary::Unary,
        variable::Variable, Expression,
    },
    statement::{
        assignment::{Assign, Get, Set},
        declaration::VariableDeclaration,
        function::FunctionStatement,
        Block, IfStatement, Statement, WhileLoop,
    },
};

/// parses an abstract syntax tree from generated tokens
pub struct AstParser {
    tokens: Vec<Token>,
    filename: String,
    current: usize,
    error_occurred: bool,
}

const MAX_PARAMETERS: usize = 8;

impl AstParser {
    pub fn new(token_container: TokenContainer) -> Self {
        let TokenContainer { scanned_tokens, filename } = token_container;

        Self {
            tokens: scanned_tokens,
            filename,
            current: 0,
            error_occurred: false,
        }
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
        if self.error_occurred {
            return Err(errors::Error::Parse(
                "Unable to parse abstract syntax tree".to_string(),
            ));
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Statement {
        let result = {
            if self.match_tokens(&[TokenType::Class]) {
                self.class_declaration()
            } else if self.match_tokens(&[TokenType::Fn]) {
                self.function_declaration("function")
            } else if self.match_tokens(&[TokenType::Let]) {
                self.var_declaration()
            }
            else if self.check_next(TokenType::ColonEqual) {
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
        let name = self
            .consume(TokenType::Identifier, "Expect class name")?
            .clone();

        let mut superclass = None;

        if self.match_tokens(&[TokenType::Colon]) {
            let superclass_name = self.consume(TokenType::Identifier, "Expect superclass name")?;
            let _ = superclass.insert(Expression::Variable(Box::new(Variable::new(
                superclass_name.clone(),
            ))));
        }

        self.consume(TokenType::NewLine, "Expect newline before class body")?;
        let mut methods = Vec::new();

        while !self.check(TokenType::End) {
            if self.match_tokens(&[TokenType::Fn]) {
                let method = self.function_declaration("method")?;
                if let Statement::FunctionStatement(function) = method {
                    methods.push(*function);
                }
                //self.consume(TokenType::NewLine, "Expect newline after end of method")?;
                continue;
            }

            return Err(self.error(&name, "Error parsing class"));
        }

        self.consume(TokenType::End, "Expect 'end' after class declaration")?;
        self.consume(
            TokenType::NewLine,
            "Expect newline after end of class declaration",
        )?;

        Ok(Statement::ClassStatement(ClassStatement::new(
            name.clone(),
            superclass,
            methods,
            name.line,
            self.filename.clone()
        )))
    }

    fn var_declaration(&mut self) -> Result<Statement, errors::Error> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name")?
            .clone();

        let mut initializer = None;

        if self.match_tokens(&[TokenType::Equal, TokenType::ColonEqual]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::NewLine,
            "Expected new line after variable declaration",
        )?;
        let line = name.line;
        let filename = self.filename.clone();
        Ok(Statement::VariableDeclaration(VariableDeclaration {
            name,
            initializer,
            line,
            filename
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
                    return Err(errors::Error::Parse(format!(
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
        let line = name.line;
        let filename = self.filename.clone();
        if let Statement::Block(body) = body {
            return Ok(Statement::FunctionStatement(Box::new(FunctionStatement {
                name,
                parameters,
                body,
                line,
                filename
            })));
        }

        let previous = self.previous().clone();
        Err(self.error(&previous, "Error parsing function"))
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

        if self.match_tokens(&[TokenType::Include]) {
            return self.include_statement();
        }

        if self.match_tokens(&[TokenType::Block]) {
            self.consume(TokenType::NewLine, "expect newline before start of block")?;
            return self.block_statement(&[TokenType::End], true);
        }

        self.expression_statement()
    }

    fn include_statement(&mut self) -> Result<Statement, errors::Error> {
        let mut files = Vec::new();

        loop {
            let file = self.expression()?;
            files.push(file);

            if self.match_tokens(&[TokenType::NewLine]) {
                break;
            }
        }

        let line = self.previous().line;
        let filename = self.filename.clone();

        Ok(Statement::Include(Include {files, line, filename}))
    }

    fn for_statement(&mut self) -> Result<Statement, errors::Error> {
        todo!()
    }

    fn if_statement(&mut self) -> Result<Statement, errors::Error> {
        let condition = self.expression()?;
        let current = self.consume(TokenType::NewLine, "Expect new line after while condition")?;
        let line = current.line;

        let then_branch = self.block_statement(&[TokenType::End, TokenType::Else], false)?;
        let mut else_branch = None;

        if self.match_tokens(&[TokenType::Else]) {
            self.consume(TokenType::NewLine, "Expect new line after while condition")?;
            else_branch = Some(self.block_statement(&[TokenType::End], true)?)
        } else {
            self.consume(TokenType::End, "Expected end after if block")?;
            self.consume(TokenType::NewLine, "Expect new line after end")?;
        }

        let filename = self.filename.clone();
        Ok(Statement::If(Box::new(IfStatement {
            condition,
            then_branch,
            else_branch,
            line,
            filename
        })))
    }

    fn return_statement(&mut self) -> Result<Statement, errors::Error> {
        let keyword = self.previous();
        let line = keyword.line;
        let filename = self.filename.clone();

        let mut value = None;

        if !self.check(TokenType::NewLine) {
            value = Some((self.expression()?, line, filename));
        }

        self.consume(TokenType::NewLine, "Expect newline after return statement")?;

        Ok(Statement::ReturnStatement(value))
    }

    fn while_statement(&mut self) -> Result<Statement, errors::Error> {
        //self.consume(TokenType::LeftParen, "Expect '(' before condition")?;
        let condition = self.expression()?;
        //self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        let current = self.consume(TokenType::NewLine, "Expect new line after while condition")?;
        let line = current.line;

        let body = self.block_statement(&[TokenType::End], true)?;
        let filename = self.filename.clone();
        Ok(Statement::WhileLoop(Box::new(WhileLoop {
            condition,
            body,
            line,
            filename
        })))
    }

    fn block_statement(
        &mut self,
        end_tokens: &[TokenType],
        consume: bool,
    ) -> Result<Statement, errors::Error> {
        let mut statements = Vec::new();
        //self.consume(TokenType::NewLine, "Expect new line before block")?;

        while !self.match_tokens(end_tokens) && !self.is_at_end() {
            statements.push(self.declaration())
        }
        let line = self.previous().line;

        self.current -= 1;

        if consume {
            self.consume(TokenType::End, "Expect end of block")?;
            self.consume(TokenType::NewLine, "expect newline after block")?;
        }

        let filename = self.filename.clone();
        Ok(Statement::Block(Block { statements, line, filename }))
    }

    fn expression_statement(&mut self) -> Result<Statement, errors::Error> {
        let expression = self.expression()?;
        let filename = self.filename.clone();

        let line = self.consume(TokenType::NewLine, "Expect newline after statement")?.line;
        Ok(Statement::ExpressionStatement((expression, line, filename)))
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
        let line = self.previous().line;
        let filename = self.filename.clone();

        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expression::Variable(variable) = &expression {
                let name = variable.name.clone();
                return Ok(Expression::Assign(Box::new(Assign { name, value, line, filename})));
            } else if let Expression::Get(get) = &expression {
                let get = *get.clone();

                return Ok(Expression::Set(Box::new(Set {
                    name: get.name,
                    object: get.object,
                    value,
                    line,
                    filename
                })));
            }

            return Err(self.error(&equals, "Invalid Assignment Target"));
        }

        Ok(expression)
    }

    fn or(&mut self) -> Result<Expression, errors::Error> {
        let expression = self.and()?;

        if self.match_tokens(&[TokenType::Or]) {
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

        if self.match_tokens(&[TokenType::And]) {
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

        if self.match_tokens(&[TokenType::EqualEqual, TokenType::NotEqual]) {
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

        if self.match_tokens(&[
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
        let mut expression = self.power()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            expression = Expression::Binary(Box::new(Binary::new(expression, right, operator)))
        }

        Ok(expression)
    }

    fn power(&mut self) -> Result<Expression, errors::Error> {
        let mut expression = self.unary()?;

        while self.match_tokens(&[TokenType::Caret]) {
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
        let line = self.previous().line;

        if self.match_tokens(&[TokenType::LeftParen]) {
            return self.finish_call(expression);
        } else if self.match_tokens(&[TokenType::Dot]) {
            let name = self
                .consume(TokenType::Identifier, "Expect name after '.'")?
                .clone();
            let mut arguments = None;
            if self.match_tokens(&[TokenType::LeftParen]) {
                arguments = Some(self.get_arguments()?);
                self.consume(TokenType::RightParen, "Expect ')' after arguments")?;
            }

            let filename = self.filename.clone();
            return Ok(Expression::Get(Box::new(Get {
                object: expression,
                name,
                arguments,
                line,
                filename
            })));
        }

        Ok(expression)
    }

    fn get_arguments(&mut self) -> Result<Vec<Expression>, errors::Error> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() > MAX_PARAMETERS {
                    let previous = self.previous().clone();
                    return Err(self.error(&previous, "Too many arguments"));
                }

                arguments.push(self.expression()?);
                if !self.match_tokens(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        Ok(arguments)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, errors::Error> {
        let arguments = self.get_arguments()?;

        let paren = self
            .consume(TokenType::RightParen, "Expect ')' after arguments")?
            .clone();
        Ok(Expression::Call(Box::new(Call::new(
            callee, paren, arguments,
        ))))
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

        // Handle literals
        if self.match_tokens(&[TokenType::None]) {
            //let token = self.previous().clone();

            return Ok(Expression::Literal(Literal::new(Object::None)));
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

        let current = self.peek().clone();
        Err(self.error(&current, "Expect Expression"))
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
        let previous = self.previous().clone();
        Err(self.error(&previous, error_message))
    }

    /// Check if the current token is of the given type
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn check_next(&self, token_type: TokenType) -> bool { 
        let next = self.peek_next();
        if let Some(token) = next {
            return token.token_type == token_type;
        }
        return false
    }

    /// Check if we are at the end of file
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Get the current token
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Get the next token
    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    } 

    /// Get the previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&mut self, token: &Token, message: &str) -> errors::Error {
        self.error_occurred = true;
        errors::Error::Parse(format!("[line: {}] (ParseError) {} ", token.line, message))
    }
}

#[cfg(test)]
mod test {}
