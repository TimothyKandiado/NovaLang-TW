use crate::language::scanner::token::Token;

use super::Block;

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Block,
}
