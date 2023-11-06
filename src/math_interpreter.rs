mod parser;
mod scanner;
mod abstract_syntax_tree;

use parser::Parser;
use scanner::Scanner;

use parser::code::OpCode;
use scanner::object::Object;

use crate::math_interpreter::abstract_syntax_tree::{parser::AstParser, interpreter::Interpreter};

#[derive(Debug)]
enum BinaryOperation {
    Add,
    Divide,
    Multiply,
    Subtract,
}

impl BinaryOperation {
    pub fn from_opcode(op_code: &OpCode) -> Option<Self> {
        match op_code {
            OpCode::Add => Some(BinaryOperation::Add),
            OpCode::Multiply => Some(BinaryOperation::Multiply),
            OpCode::Divide => Some(BinaryOperation::Divide),
            OpCode::Subtract => Some(BinaryOperation::Subtract),

            _ => None,
        }
    }
}

pub fn interpret(source: &str) {
    let scanner = Scanner::new();
    //println!("source:\n{}", source);
    let tokens = scanner.scan_tokens(source).unwrap();
    
    let mut ast_parser = AstParser::new(tokens);
    let expression = ast_parser.expression().unwrap();
    //println!("Expression: {:?}", expression);

    let interpreter = Interpreter {};
    interpreter.interpret_expression(expression);
}

fn run(instructions: Vec<OpCode>, data: Vec<Object>) -> Result<(), String> {
    let mut instructions = instructions.iter();
    let mut stack: Vec<Object> = Vec::with_capacity(256);

    loop {
        println!("stack contents: {:?}", &stack);
        let current = instructions.next();
        if current.is_none() {
            return Err(format!("Unexpected end of instructions"));
        }
        let current_instruction = current.unwrap();

        match current_instruction {
            OpCode::Constant => {
                if let Some(OpCode::ConstantIndex(constant_index)) = instructions.next() {
                    if let Some(constant_data) = data.get(*constant_index as usize) {
                        //println!("{:?}", constant_data);
                        stack.push(constant_data.to_owned());
                    } else {
                        return Err(format!("Corrupt data for instructions."));
                    }
                } else {
                    return Err(format!("Data for instructions not found"));
                }
            }

            OpCode::Negate => {
                if let Some(Object::Number(value)) = stack.pop() {
                    stack.push(Object::Number(-value));
                } else {
                    return Err(format!("Cannot negate"));
                }
            }

            OpCode::Add | OpCode::Divide | OpCode::Multiply | OpCode::Subtract => {
                if let (Some(second), Some(first)) = (stack.pop(), stack.pop()) {
                    if let Some(operation) = BinaryOperation::from_opcode(current_instruction) {
                        let result = perform_binary_operation(first, second, operation)?;
                        stack.push(result)
                    }
                }
            }

            OpCode::Return => {
                println!("stack contents: {:?}", &stack);
                break;
            }

            _ => return Err(format!("Undefined Instruction")),
        }
    }
    Ok(())
}

fn perform_binary_operation(
    first: Object,
    second: Object,
    operation: BinaryOperation,
) -> Result<Object, String> {
    match operation {
        BinaryOperation::Add => {
            if let (Object::Number(first), Object::Number(second)) = (first, second) {
                return Ok(Object::Number(first + second));
            } else {
                return Err(format!("Can't add non numbers"));
            }
        }

        BinaryOperation::Subtract => {
            if let (Object::Number(first), Object::Number(second)) = (first, second) {
                return Ok(Object::Number(first - second));
            } else {
                return Err(format!("Can't add non numbers"));
            }
        }

        BinaryOperation::Multiply => {
            if let (Object::Number(first), Object::Number(second)) = (first, second) {
                return Ok(Object::Number(first * second));
            } else {
                return Err(format!("Can't add non numbers"));
            }
        }

        BinaryOperation::Divide => {
            if let (Object::Number(first), Object::Number(second)) = (first, second) {
                return Ok(Object::Number(first / second));
            } else {
                return Err(format!("Can't add non numbers"));
            }
        }
    }
}
