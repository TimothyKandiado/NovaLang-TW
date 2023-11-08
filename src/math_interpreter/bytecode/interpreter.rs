use crate::math_interpreter::scanner::object::Object;

use super::{chunk::Chunk, code::OpCode};

enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperation {
    pub fn from_opcode(opcode: OpCode) -> Option<Self> {
        match opcode {
            OpCode::Add => Some(Self::Add),
            OpCode::Subtract => Some(Self::Subtract),
            OpCode::Multiply => Some(Self::Multiply),
            OpCode::Divide => Some(Self::Divide),

            _ => None,
        }
    }
}

pub struct BytecodeInterpreter {
    stack: Vec<Object>,
}

impl BytecodeInterpreter {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), String> {
        self.run(chunk)
    }

    fn run(&mut self, chunk: Chunk) -> Result<(), String> {
        let mut current_instruction_index = 0usize;
        loop {
            println!("stack: {:?}", &self.stack);
            let current_instruction = chunk.instructions[current_instruction_index];

            match current_instruction {
                OpCode::Constant => {
                    current_instruction_index += 1;
                    if let OpCode::ConstantIndex(constant_index) =
                        chunk.instructions[current_instruction_index]
                    {
                        let constant = chunk.constants[constant_index as usize].clone();

                        self.stack.push(constant)
                    } else {
                        return Err("corrupted constant in chunk".to_string());
                    }
                }

                OpCode::Add | OpCode::Subtract | OpCode::Divide | OpCode::Multiply => {
                    let second = self.stack.pop();
                    let first = self.stack.pop();

                    if let (Some(first), Some(second)) = (first, second) {
                        if let Some(operation) = BinaryOperation::from_opcode(current_instruction) {
                            let result = perform_binary_operation(first, second, operation)?;
                            self.stack.push(result);
                        } else {
                            return Err("Undefined binary operation".to_string());
                        }
                    } else {
                        return Err("Not enough operands for binary operation".to_string());
                    }
                }

                OpCode::Negate => {
                    if let Some(Object::Number(number)) = self.stack.pop() {
                        self.stack.push(Object::Number(-number))
                    }
                }

                OpCode::Return => {
                    println!("stack: {:?}", &self.stack);
                    break;
                }

                _ => return Err("Undefined instruction code".to_string()),
            }

            current_instruction_index += 1;
        }

        Ok(())
    }
}

fn perform_binary_operation(
    first: Object,
    second: Object,
    operation: BinaryOperation,
) -> Result<Object, String> {
    if let (Object::Number(first), Object::Number(second)) = (first, second) {
        match operation {
            BinaryOperation::Add => Ok(Object::Number(first + second)),
            BinaryOperation::Subtract => Ok(Object::Number(first - second)),
            BinaryOperation::Multiply => Ok(Object::Number(first * second)),
            BinaryOperation::Divide => Ok(Object::Number(first / second)),
        }
    } else {
        Err("Binary operations only work on numbers".to_string())
    }
}
