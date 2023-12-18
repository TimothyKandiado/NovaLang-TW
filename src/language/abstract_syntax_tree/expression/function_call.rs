use super::Expression;
use crate::language::{abstract_syntax_tree::visitor::ExpressionVisitor, scanner::object::Object};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function_id: Object,
    pub argument: Expression,
}

impl FunctionCall {
    pub fn new(function_id: Object, argument: Expression) -> Self {
        Self {
            function_id,
            argument,
        }
    }
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_function_call(self)
    }
}
