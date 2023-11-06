use crate::math_interpreter::{scanner::token::Token, abstract_syntax_tree::visitor::ExpressionVisitor};

use super::Expression;

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Expression
}

impl Unary {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_unary(self)
    }

    pub fn new(right: Expression, operator: Token) -> Self {
        Self { operator, right }
    }
}