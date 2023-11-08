mod abstract_syntax_tree;
mod bytecode;
mod scanner;

use scanner::Scanner;

use crate::math_interpreter::{abstract_syntax_tree::{
    interpreter::AstInterpreter, parser::AstParser,
}, bytecode::interpreter::BytecodeInterpreter};

use self::{abstract_syntax_tree::expression::Expression, bytecode::ast_to_bytecode::AstToBytecode};

pub fn interpret(source: &str) -> Result<String, String> {
    let expression = generate_parsed_ast(source)?;
    //println!("Expression: {:?}", expression);

    let interpreter = AstInterpreter {};
    interpreter.interpret_expression(expression)
}

pub fn interpret_with_bytecode(source: &str) -> Result<(), String> {
    let expression = generate_parsed_ast(source)?;
    let ast_to_bytecode = AstToBytecode {};
    let chunk = ast_to_bytecode.convert_expression_to_bytecode(&expression)?;

    println!("instructions: {:?}", chunk.instructions);
    println!("constants: {:?}", chunk.constants);

    let mut bytecode_interpreter = BytecodeInterpreter::new();
    bytecode_interpreter.interpret(chunk)?;
    
    Ok(())
}

fn generate_parsed_ast(source: &str) -> Result<Expression, String> {
    let scanner = Scanner::new();
    //println!("source:\n{}", source);
    let tokens = scanner.scan_tokens(source)?;

    let mut ast_parser = AstParser::new(tokens);
    ast_parser.expression()
}
