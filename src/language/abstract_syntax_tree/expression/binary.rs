use super::Expression;
use crate::language::{
    abstract_syntax_tree::visitor::ExpressionVisitor, scanner::token::Token,
};

#[derive(Debug)]
pub struct Binary {
    pub left: Expression,
    pub right: Expression,
    pub operator: Token,
}

impl Binary {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_binary(self)
    }

    pub fn new(left: Expression, right: Expression, operator: Token) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}