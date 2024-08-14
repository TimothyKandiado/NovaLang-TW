use crate::language::{Expression, StatementVisitor, Token};

use super::function::FunctionStatement;

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Expression>,
    pub methods: Vec<FunctionStatement>,
    pub line: usize,
}

impl ClassStatement {
    pub fn new(
        name: Token,
        superclass: Option<Expression>,
        methods: Vec<FunctionStatement>,
        line: usize,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
            line
        }
    }

    pub fn accept<T>(&self, visitor: &mut impl StatementVisitor<Output = T>) -> T {
        visitor.visit_class_statement(self)
    }
}
