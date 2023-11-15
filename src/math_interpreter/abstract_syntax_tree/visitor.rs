use super::expression::{
    binary::Binary, grouping::Grouping, literal::Literal, math_function::MathFunction, unary::Unary,
};

pub trait ExpressionVisitor {
    type Output;

    fn visit_binary(&self, binary: &Binary) -> Self::Output;
    fn visit_unary(&self, unary: &Unary) -> Self::Output;
    fn visit_grouping(&self, grouping: &Grouping) -> Self::Output;
    fn visit_literal(&self, literal: &Literal) -> Self::Output;
    fn visit_math_function(&self, math_function: &MathFunction) -> Self::Output;
}
