use crate::interpreter::scanner::object::Object;

use super::code::OpCode;

pub struct Chunk {
    pub instructions: Vec<OpCode>,
    pub constants: Vec<Object>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, object: Object) -> Result<(), String> {
        self.instructions.push(OpCode::Constant); // add code indicating a constant
        self.constants.push(object);

        let constant_index = self.constants.len() - 1;
        self.instructions
            .push(OpCode::ConstantIndex(constant_index as u16));

        Ok(())
    }

    pub fn append_chunk(&mut self, mut chunk: Chunk) -> Result<(), String> {
        // loop through instructions of second chunk and shift the constant indices
        for instruction_index in 0..chunk.instructions.len() {
            let instruction = &chunk.instructions[instruction_index];

            if let OpCode::ConstantIndex(constant_index) = instruction {
                let constant_index = constant_index + self.constants.len() as u16;
                chunk.instructions[instruction_index] = OpCode::ConstantIndex(constant_index)
            }
        }

        self.constants.append(&mut chunk.constants);
        self.instructions.append(&mut chunk.instructions);

        Ok(())
    }
}
