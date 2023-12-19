use crate::language::{
    abstract_syntax_tree::{expression::Expression, visitor::ExpressionVisitor},
    scanner::token::TokenType,
};

use super::{chunk::Chunk, code::OpCode};

pub struct AstToBytecode {}

impl AstToBytecode {
    pub fn convert_expression_to_bytecode(&mut self, expression: &Expression) -> Result<Chunk, String> {
        let mut chunk = self.evaluate(expression)?;
        chunk.instructions.push(OpCode::Return);
        Ok(chunk)
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Chunk, String> {
        expression.accept(self)
    }
}

impl ExpressionVisitor for AstToBytecode {
    type Output = Result<Chunk, String>;

    fn visit_binary(
        &mut self,
        binary: &crate::language::abstract_syntax_tree::expression::binary::Binary,
    ) -> Self::Output {
        let mut left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        left.append_chunk(right)?;

        match binary.operator.token_type {
            TokenType::Plus => left.instructions.push(OpCode::Add),
            TokenType::Minus => left.instructions.push(OpCode::Subtract),
            TokenType::Slash => left.instructions.push(OpCode::Divide),
            TokenType::Star => left.instructions.push(OpCode::Multiply),

            _ => return Err("unknown binary operator".to_string()),
        }

        Ok(left)
    }

    fn visit_unary(
        &mut self,
        unary: &crate::language::abstract_syntax_tree::expression::unary::Unary,
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
        &mut self,
        grouping: &crate::language::abstract_syntax_tree::expression::grouping::Grouping,
    ) -> Self::Output {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(
        &mut self,
        literal: &crate::language::abstract_syntax_tree::expression::literal::Literal,
    ) -> Self::Output {
        let object = literal.object.clone();

        let mut chunk = Chunk::new();
        chunk.add_constant(object)?;

        Ok(chunk)
    }

    fn visit_call(
        &mut self,
        _math_function: &crate::language::abstract_syntax_tree::expression::call::Call,
    ) -> Self::Output {
        todo!()
    }

    fn visit_variable(
        &mut self,
        variable: &crate::language::abstract_syntax_tree::expression::variable::Variable,
    ) -> Self::Output {
        todo!()
    }

    fn visit_assign(
        &mut self,
        assign: &crate::language::abstract_syntax_tree::statement::assignment::Assign,
    ) -> Self::Output {
        todo!()
    }

    fn visit_get(
        &mut self,
        get: &crate::language::abstract_syntax_tree::statement::assignment::Get,
    ) -> Self::Output {
        todo!()
    }

    fn visit_set(
        &mut self,
        set: &crate::language::abstract_syntax_tree::statement::assignment::Set,
    ) -> Self::Output {
        todo!()
    }
}
