use crate::language::{scanner::token::Token, abstract_syntax_tree::visitor::ExpressionVisitor};

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token
}

impl Variable {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        todo!()
    }
}