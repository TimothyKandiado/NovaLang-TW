use super::{
    expression::Expression,
    statement::{Block, Statement},
    visitor::{ExpressionVisitor, StatementVisitor},
};
use crate::language::{
    errors,
    scanner::{object::Object, token::TokenType},
};

/// A simple abstract syntax tree interpreter
pub struct AstInterpreter {}

impl AstInterpreter {
    pub fn interpret_expression(&self, expression: Expression) -> Result<String, errors::Error> {
        let result = self.evaluate(&expression)?;

        Ok(result.to_string())
    }

    pub fn interpret(&self, statements: Vec<Statement>) -> Result<(), errors::Error> {
        for statement in statements {
            self.execute(&statement)?;
        }

        Ok(())
    }

    fn execute(&self, statement: &Statement) -> Result<(), errors::Error> {
        statement.accept(self)
    }

    fn execute_block(&self, block: &Block) -> Result<(), errors::Error> {
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

    fn evaluate(&self, expression: &Expression) -> Result<Object, errors::Error> {
        expression.accept::<Result<Object, errors::Error>>(self)
    }
}

impl StatementVisitor for AstInterpreter {
    type Output = Result<(), errors::Error>;

    fn visit_if(&self, if_statement: &super::statement::IfStatement) -> Self::Output {
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

    fn visit_while(&self, while_loop: &super::statement::WhileLoop) -> Self::Output {
        while self.evaluate(&while_loop.condition)?.is_truthy() {
            self.execute(&while_loop.body)?;
        }

        Ok(())
    }

    fn visit_block(&self, block: &super::statement::Block) -> Self::Output {
        self.execute_block(block)
    }

    fn visit_function_statement(
        &self,
        function_statement: &super::statement::function::FunctionStatement,
    ) -> Self::Output {
        todo!()
    }

    fn visit_return(&self, return_statement: &Option<Expression>) -> Self::Output {
        todo!()
    }

    fn visit_var_declaration(
        &self,
        var_declaration: &super::statement::declaration::VariableDeclaration,
    ) -> Self::Output {
        todo!()
    }

    fn visit_expression_statement(&self, expression_statement: &Expression) -> Self::Output {
        self.evaluate(expression_statement)?;
        Ok(())
    }

    fn visit_none(&self) -> Self::Output {
        Err(errors::Error::InterpretError(
            "Cannot execute a nil statement".to_string(),
        ))
    }
}

impl ExpressionVisitor for AstInterpreter {
    type Output = Result<Object, errors::Error>;

    fn visit_binary(&self, binary: &super::expression::binary::Binary) -> Self::Output {
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

    fn visit_unary(&self, unary: &super::expression::unary::Unary) -> Self::Output {
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

    fn visit_grouping(&self, grouping: &super::expression::grouping::Grouping) -> Self::Output {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&self, literal: &super::expression::literal::Literal) -> Self::Output {
        Ok(literal.object.clone())
    }

    fn visit_function_call(
        &self,
        function: &super::expression::function_call::FunctionCall,
    ) -> Self::Output {
        let id = function.function_id.to_string();
        if id.as_str() == "print" {
            println!("{}", self.evaluate(&function.argument)?);
            return Ok(Object::None);
        }

        /* if let Object::Number(argument) = self.evaluate(&function.argument)? {
            let answer = match id.as_str() {
                "sin" => argument.sin(),
                "cos" => argument.cos(),
                "tan" => argument.tan(),
                "ln" => argument.ln(),
                "log" => argument.log10(),
                "sqrt" => argument.sqrt(),

                _ => return Err(errors::Error::intepret_error(&format!("No function named: {}", id))),
            };

            return Ok(Object::Number(answer));
        } */

        Err(errors::Error::intepret_error("undefined function"))
    }

    fn visit_variable(&self, variable: &super::expression::variable::Variable) -> Self::Output {
        todo!()
    }

    fn visit_assign(&self, assign: &super::statement::assignment::Assign) -> Self::Output {
        todo!()
    }

    fn visit_get(&self, get: &super::statement::assignment::Get) -> Self::Output {
        todo!()
    }

    fn visit_set(&self, set: &super::statement::assignment::Set) -> Self::Output {
        todo!()
    }
}
