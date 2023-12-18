use crate::language::{abstract_syntax_tree::expression::Expression, scanner::token::Token};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: Token,
    pub initializer: Option<Expression>,
}
