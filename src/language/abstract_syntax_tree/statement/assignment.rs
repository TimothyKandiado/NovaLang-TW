use crate::language::{
    abstract_syntax_tree::{expression::Expression, visitor::ExpressionVisitor},
    scanner::token::Token,
};

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Expression,
    pub line: usize,
    pub filename: String,
}

impl Assign {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_assign(self)
    }
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Expression,
    pub name: Token,
    pub arguments: Option<Vec<Expression>>,
    pub line: usize,
    pub filename: String,
}

impl Get {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_get(self)
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Expression,
    pub name: Token,
    pub value: Expression,
    pub line: usize,
    pub filename: String,
}

impl Set {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_set(self)
    }
}
