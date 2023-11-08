use crate::math_interpreter::{
    abstract_syntax_tree::{expression::Expression, visitor::ExpressionVisitor},
    scanner::token::TokenType,
};

use super::{chunk::Chunk, code::OpCode};

pub struct AstToBytecode {}

impl AstToBytecode {
    pub fn convert_expression_to_bytecode(&self, expression: &Expression) -> Result<Chunk, String> {
        let mut chunk = self.evaluate(expression)?;
        chunk.instructions.push(OpCode::Return);
        Ok(chunk)
    }

    fn evaluate(&self, expression: &Expression) -> Result<Chunk, String> {
        expression.accept(self)
    }
}

impl ExpressionVisitor for AstToBytecode {
    type Output = Result<Chunk, String>;

    fn visit_binary(
        &self,
        binary: &crate::math_interpreter::abstract_syntax_tree::expression::binary::Binary,
    ) -> Self::Output {
        let mut left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        left.append_chunk(right)?;

        match binary.operator.token_type {
            TokenType::Plus => left.instructions.push(OpCode::Add),
            TokenType::Minus => left.instructions.push(OpCode::Subtract),
            TokenType::Divide => left.instructions.push(OpCode::Divide),
            TokenType::Star => left.instructions.push(OpCode::Multiply),

            _ => return Err(format!("unknown binary operator")),
        }

        Ok(left)
    }

    fn visit_unary(
        &self,
        unary: &crate::math_interpreter::abstract_syntax_tree::expression::unary::Unary,
    ) -> Self::Output {
        let mut right = self.evaluate(&unary.right)?;

        match unary.operator.token_type {
            TokenType::Minus => right.instructions.push(OpCode::Negate),

            _ => {
                return Err(format!(
                    "Unknown unary operator {:?}",
                    unary.operator.token_type
                ))
            }
        }

        Ok(right)
    }

    fn visit_grouping(
        &self,
        grouping: &crate::math_interpreter::abstract_syntax_tree::expression::grouping::Grouping,
    ) -> Self::Output {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(
        &self,
        literal: &crate::math_interpreter::abstract_syntax_tree::expression::literal::Literal,
    ) -> Self::Output {
        let object = literal.object.clone();

        let mut chunk = Chunk::new();
        chunk.add_constant(object)?;

        Ok(chunk)
    }
}
