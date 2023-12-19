use crate::language::{abstract_syntax_tree::visitor::ExpressionVisitor, scanner::token::Token};

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        todo!()
    }

    pub fn new(name: Token) -> Self {
        Self { name }
    }
}
