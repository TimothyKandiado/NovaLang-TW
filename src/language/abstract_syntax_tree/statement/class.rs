use crate::language::{Token, Expression, StatementVisitor};

use super::function::FunctionStatement;

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Expression>,
    pub methods: Vec<FunctionStatement>
}

impl ClassStatement {
    pub fn new(name: Token, superclass: Option<Expression>, methods: Vec<FunctionStatement>) -> Self {
        Self { name, superclass, methods }
    }

    pub fn accept<T>(&self, visitor: &mut impl StatementVisitor<Output = T>) -> T {
        visitor.visit_class_statement(self)
    }
}