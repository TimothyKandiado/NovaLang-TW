pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

use binary::Binary;
use unary::Unary;
use literal::Literal;
use grouping::Grouping;

use super::visitor::ExpressionVisitor;

#[derive(Debug)]
pub enum Expression {
    /// left operator right
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Grouping>)
}

impl Expression {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        match self {
            Self::Binary(binary) => binary.accept(visitor),
            Self::Unary(unary) => unary.accept(visitor),
            Self::Grouping(grouping) => grouping.accept(visitor),
            Self::Literal(literal) => literal.accept(visitor),
        }
    }
}

