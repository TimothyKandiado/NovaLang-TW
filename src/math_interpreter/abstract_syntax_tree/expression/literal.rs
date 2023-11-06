use crate::math_interpreter::{scanner::object::Object, abstract_syntax_tree::visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct Literal {
    pub object: Object
}

impl Literal {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_literal(self)
    }

    pub fn new(object: Object) -> Self {
        Self { object }
    }
}