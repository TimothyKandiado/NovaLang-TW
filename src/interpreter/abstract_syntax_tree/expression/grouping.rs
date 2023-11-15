use crate::interpreter::abstract_syntax_tree::visitor::ExpressionVisitor;

use super::Expression;

#[derive(Debug)]
pub struct Grouping {
    pub expression: Expression,
}

impl Grouping {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_grouping(self)
    }

    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}
