#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Constant,
    ConstantIndex(u16),
    Add,
    Subtract,
    Divide,
    Multiply,
    Negate,
    Return,
}
