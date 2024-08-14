use crate::language::{Expression, StatementVisitor};

#[derive(Debug, Clone)]
pub struct Include {
    pub files: Vec<Expression>,
    pub line: usize,
}

impl Include {
    pub fn accept<T>(&self, visitor: &mut impl StatementVisitor<Output = T>) -> T {
        visitor.visit_include(self)
        
    }
}