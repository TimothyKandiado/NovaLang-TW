use crate::language::abstract_syntax_tree::expression::Expression;

use super::Statement;

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Statement,
}
