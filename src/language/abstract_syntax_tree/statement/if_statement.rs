use crate::language::abstract_syntax_tree::expression::Expression;

use super::Statement;

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Statement,
    pub else_branch: Option<Statement>,
}
