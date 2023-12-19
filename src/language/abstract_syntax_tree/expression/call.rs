use super::Expression;
use crate::language::{abstract_syntax_tree::visitor::ExpressionVisitor, scanner::{object::Object, token::Token}};

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Expression,
    pub paren: Token,
    pub arguments: Vec<Expression>,
}

impl Call {
    pub fn new(callee: Expression, paren: Token, arguments: Vec<Expression>) -> Self {
        Self {
            callee,
            paren,
            arguments
        }
    }
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_call(self)
    }
}
