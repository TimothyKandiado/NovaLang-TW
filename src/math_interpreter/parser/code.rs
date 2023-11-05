#[derive(Debug)]
pub enum OpCode {
    Constant,
    ConstantIndex(u16),
    Negate,
    Subtract,
    Add,
    Multiply,
    Divide,
    Return,
}
