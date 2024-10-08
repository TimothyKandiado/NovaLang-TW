use super::Statement;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub line: usize,
    pub filename: String,
}
