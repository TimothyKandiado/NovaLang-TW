
use super::{expression::{
    binary::Binary, grouping::Grouping, literal::Literal, function_call::FunctionCall, unary::Unary, variable::Variable, Expression,
}, statement::{assignment::{Assign, Set, Get}, IfStatement, WhileLoop, Block, function::FunctionStatement, declaration::VariableDeclaration}};

pub trait ExpressionVisitor {
    type Output;

    fn visit_binary(&self, binary: &Binary) -> Self::Output;
    fn visit_unary(&self, unary: &Unary) -> Self::Output;
    fn visit_grouping(&self, grouping: &Grouping) -> Self::Output;
    fn visit_literal(&self, literal: &Literal) -> Self::Output;
    fn visit_function_call(&self, math_function: &FunctionCall) -> Self::Output;
    fn visit_variable(&self, variable: &Variable) -> Self::Output;
    fn visit_assign(&self, assign: &Assign) -> Self::Output;
    fn visit_get(&self, get: &Get) -> Self::Output;
    fn visit_set(&self, set: &Set) -> Self::Output;
}

pub trait StatementVisitor {
    type Output;

    fn visit_none(&self) -> Self::Output;
    fn visit_if(&self, if_statement: &IfStatement) -> Self::Output;
    fn visit_while(&self, while_loop: &WhileLoop) -> Self::Output;
    fn visit_block(&self, block: &Block) -> Self::Output;
    fn visit_function_statement(&self, function_statement: &FunctionStatement) -> Self::Output;
    fn visit_return(&self, return_statement: &Option<Expression>) -> Self::Output;
    fn visit_var_declaration(&self, var_declaration: &VariableDeclaration) -> Self::Output;
    fn visit_expression_statement(&self, expression_statement: &Expression) -> Self::Output;
}
