pub mod assignment;
pub mod block;
pub mod class;
pub mod declaration;
pub mod function;
pub mod if_statement;
pub mod while_loop;
pub mod include;

pub use block::Block;
pub use if_statement::IfStatement;
pub use while_loop::WhileLoop;
pub use include::Include;

use self::{class::ClassStatement, declaration::VariableDeclaration, function::FunctionStatement};

use super::{expression::Expression, visitor::StatementVisitor};

#[derive(Debug, Clone)]
pub enum Statement {
    None,
    If(Box<IfStatement>),
    WhileLoop(Box<WhileLoop>),
    Block(Block),
    FunctionStatement(Box<FunctionStatement>),
    ReturnStatement(Option<Expression>),
    VariableDeclaration(VariableDeclaration),
    ExpressionStatement(Expression),
    ClassStatement(ClassStatement),
    Include(Include),
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut impl StatementVisitor<Output = T>) -> T {
        match self {
            Self::None => visitor.visit_none(),
            Self::If(if_statement) => visitor.visit_if(if_statement),
            Self::WhileLoop(while_loop) => visitor.visit_while(while_loop),
            Self::Block(block) => visitor.visit_block(block),
            Self::FunctionStatement(function_statement) => {
                visitor.visit_function_statement(function_statement)
            }
            Self::ReturnStatement(return_values) => visitor.visit_return(return_values),
            Self::VariableDeclaration(var_declaration) => {
                visitor.visit_var_declaration(var_declaration)
            }
            Self::ExpressionStatement(expression_statement) => {
                visitor.visit_expression_statement(expression_statement)
            }
            Self::ClassStatement(class_statement) => class_statement.accept(visitor),
            Self::Include(include) => include.accept(visitor),
        }
    }
}
