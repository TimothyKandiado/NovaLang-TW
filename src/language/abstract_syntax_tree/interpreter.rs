use std::sync::{Arc, RwLock};

use super::{
    environment::Environment,
    expression::Expression,
    statement::{Block, Statement},
    visitor::{ExpressionVisitor, StatementVisitor},
};
use crate::language::{
    errors,
    scanner::{object::Object, token::TokenType},
};

/// A simple abstract syntax tree interpreter
pub struct AstInterpreter {
    environment: Arc<RwLock<Environment>>,
}

impl AstInterpreter {
    pub fn new() -> Self {
        Self {
            environment: Arc::new(RwLock::new(Environment::new())),
        }
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> Result<String, errors::Error> {
        let result = self.evaluate(&expression)?;

        Ok(result.to_string())
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), errors::Error> {
        for statement in statements {
            self.execute(&statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), errors::Error> {
        statement.accept(self)
    }

    fn execute_block(&mut self, block: &Block) -> Result<(), errors::Error> {
        // create new environment
        let mut result = Ok(());

        for statement in &block.statements {
            result = self.execute(statement);
            if result.is_err() {
                break;
            }
        }

        result
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Object, errors::Error> {
        expression.accept::<Result<Object, errors::Error>>(self)
    }

    pub fn print_environment(&self) {
        let environment = Arc::clone(&self.environment);
        let env_reader = environment.read();

        if let Ok(env_reader) = env_reader {
            println!("=== Environment ===");
            println!("{}", (*env_reader).to_string());
            println!("===================");
        }
    }
}

impl StatementVisitor for AstInterpreter {
    type Output = Result<(), errors::Error>;

    fn visit_if(&mut self, if_statement: &super::statement::IfStatement) -> Self::Output {
        let condition = self.evaluate(&if_statement.condition)?;

        if condition.is_truthy() {
            self.execute(&if_statement.then_branch)?;
        } else {
            if let Some(else_branch) = &if_statement.else_branch {
                self.execute(else_branch)?;
            }
        }

        Ok(())
    }

    fn visit_while(&mut self, while_loop: &super::statement::WhileLoop) -> Self::Output {
        while self.evaluate(&while_loop.condition)?.is_truthy() {
            self.execute(&while_loop.body)?;
        }

        Ok(())
    }

    fn visit_block(&mut self, block: &super::statement::Block) -> Self::Output {
        self.execute_block(block)
    }

    fn visit_function_statement(
        &mut self,
        function_statement: &super::statement::function::FunctionStatement,
    ) -> Self::Output {
        todo!()
    }

    fn visit_return(&mut self, return_statement: &Option<Expression>) -> Self::Output {
        todo!()
    }

    fn visit_var_declaration(
        &mut self,
        var_declaration: &super::statement::declaration::VariableDeclaration,
    ) -> Self::Output {
        let mut value = Arc::new(RwLock::new(Object::None));

        if let Some(initializer) = &var_declaration.initializer {
            let initializer = self.evaluate(initializer)?;
            value = Arc::new(RwLock::new(initializer.clone()));
        }

        let env_writer = self.environment.write();
        if let Ok(mut env_writer) = env_writer {
            (*env_writer).declare_value(var_declaration.name.object.to_string().as_str(), value)
        } else {
            return Err(errors::Error::RuntimeError(
                env_writer.unwrap_err().to_string(),
            ));
        }

        Ok(())
    }

    fn visit_expression_statement(&mut self, expression_statement: &Expression) -> Self::Output {
        self.evaluate(expression_statement)?;
        Ok(())
    }

    fn visit_none(&mut self) -> Self::Output {
        Err(errors::Error::InterpretError(
            "Cannot execute a nil statement".to_string(),
        ))
    }
}

impl ExpressionVisitor for AstInterpreter {
    type Output = Result<Object, errors::Error>;

    fn visit_binary(&mut self, binary: &super::expression::binary::Binary) -> Self::Output {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.token_type {
            TokenType::Plus => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left + right));
                }

                Err(errors::Error::intepret_error("Cannot add non numbers"))
            }

            TokenType::Minus => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left - right));
                }

                Err(errors::Error::intepret_error("Cannot subtract non numbers"))
            }

            TokenType::Slash => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left / right));
                }

                Err(errors::Error::intepret_error("Cannot divide non numbers"))
            }

            TokenType::Star => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left * right));
                }

                Err(errors::Error::intepret_error("Cannot multiply non numbers"))
            }

            TokenType::Or => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left.is_truthy() || right.is_truthy()))
            }

            TokenType::And => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left.is_truthy() && right.is_truthy()))
            }

            TokenType::EqualEqual => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left == right))
            }

            TokenType::Greater => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left > right))
            }

            TokenType::GreaterEqual => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left >= right))
            }

            TokenType::Less => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left < right))
            }

            TokenType::LessEqual => {
                let left = self.evaluate(&binary.left)?;
                let right = self.evaluate(&binary.right)?;

                Ok(Object::Bool(left <= right))
            }

            _ => Err(errors::Error::intepret_error(&format!(
                "Undefined binary operation: {:?}",
                binary.operator.token_type
            ))),
        }
    }

    fn visit_unary(&mut self, unary: &super::expression::unary::Unary) -> Self::Output {
        let right = self.evaluate(&unary.right)?;

        match unary.operator.token_type {
            TokenType::Minus => {
                if let Object::Number(right) = right {
                    return Ok(Object::Number(-right));
                }

                Err(errors::Error::intepret_error("Cannot negate a non number"))
            }

            _ => Err(errors::Error::intepret_error(&format!(
                "Undefined Unary Operation : {:?}",
                unary.operator.token_type
            ))),
        }
    }

    fn visit_grouping(&mut self, grouping: &super::expression::grouping::Grouping) -> Self::Output {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&mut self, literal: &super::expression::literal::Literal) -> Self::Output {
        Ok(literal.object.clone())
    }

    fn visit_call(
        &mut self,
        call: &super::expression::call::Call,
    ) -> Self::Output {
        let callee = self.evaluate(&call.callee)?;
        let mut arguments = Vec::new();

        for argument in &call.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        if let Object::Callable(callable) = callee {
            if callable.arity() != arguments.len() as i8 && callable.arity() != -1 {
                return Err(errors::Error::intepret_error("too many function arguments"))
            }

            return callable.call(self, &arguments);
        }

        

        Err(errors::Error::intepret_error("undefined function"))
    }

    fn visit_variable(&mut self, variable: &super::expression::variable::Variable) -> Self::Output {
        todo!()
    }

    fn visit_assign(&mut self, assign: &super::statement::assignment::Assign) -> Self::Output {
        todo!()
    }

    fn visit_get(&mut self, get: &super::statement::assignment::Get) -> Self::Output {
        todo!()
    }

    fn visit_set(&mut self, set: &super::statement::assignment::Set) -> Self::Output {
        todo!()
    }

}
