pub mod binary;
pub mod function_call;
pub mod grouping;
pub mod literal;
pub mod unary;
pub mod variable;

use binary::Binary;
use grouping::Grouping;
use literal::Literal;
use unary::Unary;

use self::{function_call::FunctionCall, variable::Variable};

use super::{
    statement::assignment::{Assign, Get, Set},
    visitor::ExpressionVisitor,
};

#[derive(Debug, Clone)]
pub enum Expression {
    /// left operator right
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Grouping>),
    FunctionCall(Box<FunctionCall>),
    Variable(Box<Variable>),
    Assign(Box<Assign>),
    Get(Box<Get>),
    Set(Box<Set>),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &impl ExpressionVisitor<Output = T>) -> T {
        match self {
            Self::Binary(binary) => binary.accept(visitor),
            Self::Unary(unary) => unary.accept(visitor),
            Self::Grouping(grouping) => grouping.accept(visitor),
            Self::Literal(literal) => literal.accept(visitor),
            Self::FunctionCall(math_function) => math_function.accept(visitor),
            Self::Variable(variable_expression) => variable_expression.accept(visitor),
            Self::Assign(assign) => assign.accept(visitor),
            Self::Get(get) => get.accept(visitor),
            Self::Set(set) => set.accept(visitor),
        }
    }
}
