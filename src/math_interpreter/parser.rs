pub mod code;

use self::code::OpCode;

use super::scanner::{
    object::Object,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    pub fn parse_instructions(
        mut self,
        tokens: Vec<Token>,
        instructions: &mut Vec<OpCode>,
        data: &mut Vec<Object>,
    ) {
        self.tokens = tokens;
        while !self.is_at_end() {
            self.parse_instruction(instructions, data)
        }
    }

    fn parse_instruction(&mut self, instructions: &mut Vec<OpCode>, data: &mut Vec<Object>) {
        self.start = self.current;
        let token = self.advance().unwrap();
        let token_type = token.token_type;

        match token_type {
            TokenType::Number => {
                instructions.push(OpCode::Constant);
                data.push(token.object.clone());
                let constant_index = data.len() - 1;
                instructions.push(OpCode::ConstantIndex(constant_index as u16));
            }

            TokenType::Divide => self
                .emit_binary_op(instructions, data, OpCode::Divide)
                .unwrap(),
            TokenType::Star => self
                .emit_binary_op(instructions, data, OpCode::Multiply)
                .unwrap(),
            TokenType::Plus => self
                .emit_binary_op(instructions, data, OpCode::Add)
                .unwrap(),
            //TokenType::Minus => self.emit_binary_op(instructions, data, OpCode::Su).unwrap(),
            TokenType::Eof => {
                instructions.push(OpCode::Return);
            }

            _ => {}
        }
    }

    fn emit_binary_op(
        &mut self,
        instructions: &mut Vec<OpCode>,
        data: &mut Vec<Object>,
        operation: OpCode,
    ) -> Result<(), String> {
        if let Some(next) = self.peek() {
            match next.token_type {
                TokenType::Number => {
                    instructions.push(OpCode::Constant);
                    data.push(next.object.clone());

                    let constant_index = data.len() - 1;
                    instructions.push(OpCode::ConstantIndex(constant_index as u16));
                    instructions.push(operation);
                    self.advance();
                }

                _ => return Err(format!("Cannot perform binary operation")),
            }
        }

        Ok(())
    }

    fn peek(&self) -> Option<&Token> {
        if self.is_at_end() {
            return None;
        }

        let token = &self.tokens[self.current];
        Some(token)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.is_at_end() {
            return None;
        }

        self.current += 1;
        let token = &self.tokens[self.current - 1];
        Some(token)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
