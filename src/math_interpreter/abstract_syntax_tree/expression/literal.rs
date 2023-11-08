use crate::math_interpreter::{
    abstract_syntax_tree::visitor::ExpressionVisitor, scanner::object::Object,
};

#[derive(Debug)]
pub struct Literal {
    pub object: Object,
}

impl Literal {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_literal(self)
    }

    pub fn new(object: Object) -> Self {
        Self { object }
    }
}
