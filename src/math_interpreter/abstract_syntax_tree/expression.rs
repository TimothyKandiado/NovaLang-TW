pub mod binary;
pub mod grouping;
pub mod literal;
pub mod math_function;
pub mod unary;

use binary::Binary;
use grouping::Grouping;
use literal::Literal;
use unary::Unary;

use self::math_function::MathFunction;

use super::visitor::ExpressionVisitor;

#[derive(Debug)]
pub enum Expression {
    /// left operator right
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Grouping>),
    MathFunction(Box<MathFunction>),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        match self {
            Self::Binary(binary) => binary.accept(visitor),
            Self::Unary(unary) => unary.accept(visitor),
            Self::Grouping(grouping) => grouping.accept(visitor),
            Self::Literal(literal) => literal.accept(visitor),
            Self::MathFunction(math_function) => math_function.accept(visitor),
        }
    }
}
