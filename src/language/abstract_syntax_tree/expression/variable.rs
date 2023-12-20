use crate::language::{abstract_syntax_tree::visitor::ExpressionVisitor, scanner::token::Token};

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_variable(self)
    }

    pub fn new(name: Token) -> Self {
        Self { name }
    }
}
