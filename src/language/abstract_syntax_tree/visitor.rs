use crate::language::class::ClassStatement;

use super::{
    expression::{
        binary::Binary, call::Call, grouping::Grouping, literal::Literal, unary::Unary,
        variable::Variable, Expression,
    },
    statement::{
        assignment::{Assign, Get, Set},
        declaration::VariableDeclaration,
        function::FunctionStatement,
        Block, IfStatement, WhileLoop,
    },
};

pub trait ExpressionVisitor {
    type Output;

    fn visit_binary(&mut self, binary: &Binary) -> Self::Output;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;
    fn visit_call(&mut self, math_function: &Call) -> Self::Output;
    fn visit_variable(&mut self, variable: &Variable) -> Self::Output;
    fn visit_assign(&mut self, assign: &Assign) -> Self::Output;
    fn visit_get(&mut self, get: &Get) -> Self::Output;
    fn visit_set(&mut self, set: &Set) -> Self::Output;
}

pub trait StatementVisitor {
    type Output;

    fn visit_none(&mut self) -> Self::Output;
    fn visit_if(&mut self, if_statement: &IfStatement) -> Self::Output;
    fn visit_while(&mut self, while_loop: &WhileLoop) -> Self::Output;
    fn visit_block(&mut self, block: &Block) -> Self::Output;
    fn visit_function_statement(&mut self, function_statement: &FunctionStatement) -> Self::Output;
    fn visit_return(&mut self, return_statement: &Option<Expression>) -> Self::Output;
    fn visit_var_declaration(&mut self, var_declaration: &VariableDeclaration) -> Self::Output;
    fn visit_expression_statement(&mut self, expression_statement: &Expression) -> Self::Output;
    fn visit_class_statement(&mut self, class_statement: &ClassStatement) -> Self::Output;
}
