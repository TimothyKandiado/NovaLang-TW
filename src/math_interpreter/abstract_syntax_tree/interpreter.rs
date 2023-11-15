use super::{expression::Expression, visitor::ExpressionVisitor};
use crate::math_interpreter::scanner::{object::Object, token::TokenType};

/// A simple abstract syntax tree interpreter
pub struct AstInterpreter {}

impl AstInterpreter {
    pub fn interpret_expression(&self, expression: Expression) -> Result<String, String> {
        let result = self.evaluate(&expression)?;

        Ok(result.to_string())
    }

    fn evaluate(&self, expression: &Expression) -> Result<Object, String> {
        expression.accept::<Result<Object, String>>(self)
    }
}

impl ExpressionVisitor for AstInterpreter {
    type Output = Result<Object, String>;

    fn visit_binary(&self, binary: &super::expression::binary::Binary) -> Self::Output {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.token_type {
            TokenType::Plus => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left + right));
                }

                Err("Cannot add non numbers".to_string())
            }

            TokenType::Minus => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left - right));
                }

                Err("Cannot subtract non numbers".to_string())
            }

            TokenType::Divide => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left / right));
                }

                Err("Cannot divide non numbers".to_string())
            }

            TokenType::Star => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left * right));
                }

                Err("Cannot multiply non numbers".to_string())
            }

            _ => Err(format!(
                "Undefined binary operation: {:?}",
                binary.operator.token_type
            )),
        }
    }

    fn visit_unary(&self, unary: &super::expression::unary::Unary) -> Self::Output {
        let right = self.evaluate(&unary.right)?;

        match unary.operator.token_type {
            TokenType::Minus => {
                if let Object::Number(right) = right {
                    return Ok(Object::Number(-right));
                }

                Err("Cannot negate a non number".to_string())
            }

            _ => Err(format!(
                "Undefined Unary Operation : {:?}",
                unary.operator.token_type
            )),
        }
    }

    fn visit_grouping(&self, grouping: &super::expression::grouping::Grouping) -> Self::Output {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&self, literal: &super::expression::literal::Literal) -> Self::Output {
        Ok(literal.object.clone())
    }

    fn visit_math_function(
        &self,
        math_function: &super::expression::math_function::MathFunction,
    ) -> Self::Output {
        let id = math_function.function_id.to_string();
        if let Object::Number(argument) = self.evaluate(&math_function.argument)? {
            let answer = match id.as_str() {
                "sin" => argument.sin(),
                "cos" => argument.cos(),
                "tan" => argument.tan(),
                "ln" => argument.ln(),
                "log" => argument.log10(),
                "sqrt" => argument.sqrt(),

                _ => return Err(format!("No mathematical function named: {}", id)),
            };

            return Ok(Object::Number(answer));
        }

        return Err(format!("argument needs to be a number"));
    }
}
