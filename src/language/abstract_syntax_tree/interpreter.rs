use std::{
    io::{self, Write},
    sync::{Arc, RwLock},
};

use super::{
    environment::Environment,
    expression::Expression,
    statement::{Block, Statement},
    visitor::{ExpressionVisitor, StatementVisitor},
};
use crate::language::{
    errors,
    scanner::{
        object::{Callable, NativeCall, Object, WrappedObject},
        token::TokenType,
    },
};

/// A simple abstract syntax tree interpreter
pub struct AstInterpreter {
    environment: Arc<RwLock<Environment>>,
}

impl Default for AstInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl AstInterpreter {
    pub fn new() -> Self {
        let mut global_env = Environment::new();
        AstInterpreter::load_native_functions(&mut global_env);

        let global = Arc::new(RwLock::new(global_env));
        Self {
            environment: Arc::new(RwLock::new(Environment::with_parent(global))),
        }
    }

    fn load_native_functions(environment: &mut Environment) {
        let println = |_interpreter: &mut AstInterpreter,
                       arguments: &Vec<WrappedObject>|
         -> Result<WrappedObject, errors::Error> {
            for argument in arguments {
                let binding = argument.read();
                let argument = binding.unwrap();
                print!("{}", *argument);
            }
            println!();

            Ok(Object::None.wrap())
        };

        let println_object = Object::Callable(Callable::NativeCall(NativeCall::new(-1, println)));

        environment.declare_value("println", println_object.wrap());

        let print = |_interpreter: &mut AstInterpreter,
                     arguments: &Vec<WrappedObject>|
         -> Result<WrappedObject, errors::Error> {
            for argument in arguments {
                let binding = argument.read();
                let argument = binding.unwrap();
                print!("{}", *argument);
            }
            let _ = io::stdout().flush();

            Ok(Object::None.wrap())
        };

        let print_object = Object::Callable(Callable::NativeCall(NativeCall::new(-1, print)));

        environment.declare_value("print", print_object.wrap())
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

    fn execute_block(
        &mut self,
        block: &Block,
        new_environment: Arc<RwLock<Environment>>,
    ) -> Result<(), errors::Error> {
        // create new environment
        let mut result = Ok(());
        let previous_environment = Arc::clone(&self.environment);

        self.environment = new_environment;

        for statement in &block.statements {
            result = self.execute(statement);
            if result.is_err() {
                break;
            }
        }

        self.environment = previous_environment;

        result
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<WrappedObject, errors::Error> {
        expression.accept::<Result<WrappedObject, errors::Error>>(self)
    }

    pub fn print_environment(&self) {
        let environment = Arc::clone(&self.environment);
        let env_reader = environment.read();

        if let Ok(env_reader) = env_reader {
            println!("=== Environment ===");
            println!("{}", (*env_reader));
            println!("===================");
        }
    }
}

impl StatementVisitor for AstInterpreter {
    type Output = Result<(), errors::Error>;

    fn visit_if(&mut self, if_statement: &super::statement::IfStatement) -> Self::Output {
        let condition = self.evaluate(&if_statement.condition)?;

        let condition = condition.read();
        if let Ok(condition) = condition {
            if condition.is_truthy() {
                self.execute(&if_statement.then_branch)?;
            } else if let Some(else_branch) = &if_statement.else_branch {
                self.execute(else_branch)?;
            }
        } else {
            let _unused = condition.unwrap();
        }

        Ok(())
    }

    fn visit_while(&mut self, while_loop: &super::statement::WhileLoop) -> Self::Output {
        while self
            .evaluate(&while_loop.condition)?
            .read()
            .unwrap()
            .is_truthy()
        {
            self.execute(&while_loop.body)?;
        }

        Ok(())
    }

    fn visit_block(&mut self, block: &super::statement::Block) -> Self::Output {
        let previous_environment = Arc::clone(&self.environment);
        let new_environment = Arc::new(RwLock::new(Environment::with_parent(previous_environment)));

        self.execute_block(block, new_environment)
    }

    fn visit_function_statement(
        &mut self,
        _function_statement: &super::statement::function::FunctionStatement,
    ) -> Self::Output {
        todo!()
    }

    fn visit_return(&mut self, _return_statement: &Option<Expression>) -> Self::Output {
        todo!()
    }

    fn visit_var_declaration(
        &mut self,
        var_declaration: &super::statement::declaration::VariableDeclaration,
    ) -> Self::Output {
        let mut value = Arc::new(RwLock::new(Object::None));

        if let Some(initializer) = &var_declaration.initializer {
            let initializer = self.evaluate(initializer)?;
            value = Arc::new(RwLock::new(initializer.read().unwrap().clone()));
        }

        let env_writer = self.environment.write();
        if let Ok(mut env_writer) = env_writer {
            (*env_writer).declare_value(var_declaration.name.object.to_string().as_str(), value)
        } else {
            return Err(errors::Error::Runtime(env_writer.unwrap_err().to_string()));
        }

        Ok(())
    }

    fn visit_expression_statement(&mut self, expression_statement: &Expression) -> Self::Output {
        self.evaluate(expression_statement)?;
        Ok(())
    }

    fn visit_none(&mut self) -> Self::Output {
        Err(errors::Error::Interpret(
            "Cannot execute a nil statement".to_string(),
        ))
    }
}

impl ExpressionVisitor for AstInterpreter {
    type Output = Result<WrappedObject, errors::Error>;

    fn visit_binary(&mut self, binary: &super::expression::binary::Binary) -> Self::Output {
        let left_binding = self.evaluate(&binary.left)?;
        let right_binding = self.evaluate(&binary.right)?;

        let left = left_binding.read().unwrap();
        let right = right_binding.read().unwrap();

        match binary.operator.token_type {
            TokenType::Plus => {
                // add numbers
                if let (Object::Number(left), Object::Number(right)) = ((&*left), &(*right)) {
                    return Ok(Arc::new(RwLock::new(Object::Number(left + right))));
                }

                // concanate strings
                if let Object::String(left) = &*left {
                    let right = (*right).to_string();

                    return Ok(Arc::new(RwLock::new(Object::String(format!(
                        "{}{}",
                        left, right
                    )))));
                }

                if let Object::String(right) = &*right {
                    let left = (*left).to_string();

                    return Ok(Arc::new(RwLock::new(Object::String(format!(
                        "{}{}",
                        left, right
                    )))));
                }

                Err(errors::Error::intepret_error(
                    "Can only add numbers or concanate strings",
                ))
            }

            TokenType::Minus => {
                if let (Object::Number(left), Object::Number(right)) = ((&*left), &(*right)) {
                    return Ok(Object::Number(left - right).wrap());
                }

                Err(errors::Error::intepret_error("Cannot subtract non numbers"))
            }

            TokenType::Slash => {
                if let (Object::Number(left), Object::Number(right)) = ((&*left), &(*right)) {
                    return Ok(Object::Number(left / right).wrap());
                }

                Err(errors::Error::intepret_error("Cannot divide non numbers"))
            }

            TokenType::Star => {
                if let (Object::Number(left), Object::Number(right)) = ((&*left), &(*right)) {
                    return Ok(Object::Number(left * right).wrap());
                }

                Err(errors::Error::intepret_error("Cannot multiply non numbers"))
            }

            TokenType::Or => Ok(Object::Bool((*left).is_truthy() || (*right).is_truthy()).wrap()),

            TokenType::And => Ok(Object::Bool((*left).is_truthy() && (*right).is_truthy()).wrap()),

            TokenType::EqualEqual => Ok(Object::Bool(*left == *right).wrap()),

            TokenType::Greater => Ok(Object::Bool(*left > *right).wrap()),

            TokenType::GreaterEqual => Ok(Object::Bool(*left >= *right).wrap()),

            TokenType::Less => Ok(Object::Bool(*left < *right).wrap()),

            TokenType::LessEqual => Ok(Object::Bool(*left <= *right).wrap()),

            _ => Err(errors::Error::intepret_error(&format!(
                "Undefined binary operation: {:?}",
                binary.operator.token_type
            ))),
        }
    }

    fn visit_unary(&mut self, unary: &super::expression::unary::Unary) -> Self::Output {
        let binding = self.evaluate(&unary.right)?;
        let right = binding.read().unwrap();

        match unary.operator.token_type {
            TokenType::Minus => {
                if let Object::Number(right) = right.to_owned() {
                    return Ok(Object::Number(-right).wrap());
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
        Ok(literal.object.clone().wrap())
    }

    fn visit_call(&mut self, call: &super::expression::call::Call) -> Self::Output {
        let callee_binding = self.evaluate(&call.callee)?;
        let callee = callee_binding.read().unwrap();

        let mut arguments = Vec::new();

        for argument in &call.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        if let Object::Callable(callable) = &(*callee) {
            if callable.arity() != arguments.len() as i8 && callable.arity() != -1 {
                return Err(errors::Error::intepret_error("too many function arguments"));
            }

            return callable.call(self, &arguments);
        }

        Err(errors::Error::intepret_error("undefined function"))
    }

    fn visit_variable(&mut self, variable: &super::expression::variable::Variable) -> Self::Output {
        let env_reader = self.environment.read();
        if let Ok(env_reader) = env_reader {
            let object = env_reader.get_value(variable.name.object.to_string().as_str());
            return Ok(object);
        }

        Err(errors::Error::Runtime("Error retrieving value".to_string()))
    }

    fn visit_assign(&mut self, assign: &super::statement::assignment::Assign) -> Self::Output {
        let value = self.evaluate(&assign.value)?;

        let env_writer = self.environment.write();
        if let Ok(mut env_writer) = env_writer {
            //let value = Arc::new(RwLock::new(value));
            (*env_writer).set_value(assign.name.object.to_string().as_str(), value)?;
        } else {
            let err = env_writer.unwrap_err();
            return Err(errors::Error::Runtime(err.to_string()));
        }

        Ok(Object::None.wrap())
    }

    fn visit_get(&mut self, _get: &super::statement::assignment::Get) -> Self::Output {
        todo!()
    }

    fn visit_set(&mut self, _set: &super::statement::assignment::Set) -> Self::Output {
        todo!()
    }
}
