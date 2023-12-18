use crate::language::{scanner::token::Token, abstract_syntax_tree::expression::Expression};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: Token,
    pub initializer: Option<Expression>
}