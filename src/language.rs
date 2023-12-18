mod abstract_syntax_tree;
mod bytecode;
mod scanner;
mod errors;

pub use scanner::Scanner;
pub use scanner::token::debug_print_tokens;

use crate::language::{
    abstract_syntax_tree::{interpreter::AstInterpreter, parser::AstParser},
    bytecode::interpreter::BytecodeInterpreter,
};

use self::{
    abstract_syntax_tree::{expression::Expression, statement::Statement}, bytecode::ast_to_bytecode::AstToBytecode,
};

pub fn interpret(source: &str) -> Result<(), errors::Error> {
    let statements = generate_parsed_ast(source)?;
    //println!("Expression: {:?}", expression);

    let interpreter = AstInterpreter {};
    //println!("{:?}", &statements);
    interpreter.interpret(statements)?;
    Ok(())
}

pub fn interpret_with_bytecode(source: &str) -> Result<String, errors::Error> {
    todo!()
    /* let expression = generate_parsed_ast(source)?;
    let ast_to_bytecode = AstToBytecode {};
    let chunk = ast_to_bytecode.convert_expression_to_bytecode(&expression)?;

    let mut bytecode_interpreter = BytecodeInterpreter::new();
    let result = bytecode_interpreter.interpret(chunk)?;

    Ok(result.to_string()) */
}

fn generate_parsed_ast(source: &str) -> Result<Vec<Statement>, errors::Error> {
    let scanner = Scanner::new();
    //println!("source:\n{}", source);
    let tokens = scanner.scan_tokens(source)?;

    let mut ast_parser = AstParser::new(tokens);
    ast_parser.parse_ast()
}
