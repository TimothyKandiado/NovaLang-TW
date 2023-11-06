use crate::math_interpreter::scanner::{object::Object, token::TokenType};
use super::{expression::Expression, visitor::ExpressionVisitor};

pub struct Interpreter {

}

impl Interpreter {
    pub fn interpret_expression(&self, expression: Expression) {
        let result = self.evaluate(&expression).unwrap();

        println!("result: {}", result.to_string())
    }

    fn evaluate(&self, expression: &Expression) -> Result<Object, String> {
        expression.accept::<Result<Object, String>>(self)
    }
}

impl ExpressionVisitor for Interpreter {
    type Output = Result<Object, String>;

    fn visit_binary(&self, binary: &super::expression::binary::Binary) -> Self::Output {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.token_type {
            TokenType::Plus => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left + right))
                }

                Err(format!("Cannot add non numbers"))
            }

            TokenType::Minus => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left - right))
                }

                Err(format!("Cannot subtract non numbers"))
            }

            TokenType::Divide => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left / right))
                }

                Err(format!("Cannot divide non numbers"))
            }

            TokenType::Star => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    return Ok(Object::Number(left * right))
                }

                Err(format!("Cannot multiply non numbers"))
            }

            _ => {
                Err (format!("Undefined binary operation: {:?}", binary.operator.token_type))
            }
        }
    }

    fn visit_unary(&self, unary: &super::expression::unary::Unary) -> Self::Output {
        let right = self.evaluate(&unary.right)?;

        match unary.operator.token_type {
            TokenType::Minus => {
                if let Object::Number(right) = right {
                    return Ok(Object::Number(-right));
                }

                Err(format!("Cannot negate a non number"))
            }

            _ => {
                Err(format!("Undefined Unary Operation : {:?}", unary.operator.token_type))
            }
        }
    }

    fn visit_grouping(&self, grouping: &super::expression::grouping::Grouping) -> Self::Output {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&self, literal: &super::expression::literal::Literal) -> Self::Output {
        Ok(literal.object.clone())
    }
}