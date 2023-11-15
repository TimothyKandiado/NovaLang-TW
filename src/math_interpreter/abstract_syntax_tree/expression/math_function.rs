use super::Expression;
use crate::math_interpreter::{
    abstract_syntax_tree::visitor::ExpressionVisitor, scanner::object::Object,
};

#[derive(Debug)]
pub struct MathFunction {
    pub function_id: Object,
    pub argument: Expression,
}

impl MathFunction {
    pub fn new(function_id: Object, argument: Expression) -> Self {
        Self {
            function_id,
            argument,
        }
    }
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_math_function(self)
    }
}
