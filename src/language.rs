mod abstract_syntax_tree;
pub mod errors;
mod scanner;

pub use abstract_syntax_tree::{interpreter::AstInterpreter, parser::AstParser};
pub use scanner::token::debug_print_tokens;
pub use scanner::Scanner;

pub use scanner::object::{Callable, NativeCall, Object};
pub use scanner::token::{Token, TokenType};

pub use abstract_syntax_tree::{expression::*, statement::*, visitor::*};

pub fn interpret(source: &str, filename: &str) -> Result<(), errors::Error> {
    let statements = generate_parsed_ast(source, filename)?;
    //println!("Expression: {:?}", expression);

    let mut interpreter = AstInterpreter::new();
    //println!("{:?}", &statements);
    interpreter.interpret(statements)?;
    Ok(())
}

pub fn generate_parsed_ast(source: &str, filename: &str) -> Result<Vec<Statement>, errors::Error> {
    let scanner = Scanner::new();
    //println!("source:\n{}", source);
    let tokens = scanner.scan_tokens_with_filename(source, filename)?;

    let ast_parser = AstParser::new(tokens);
    ast_parser.parse_ast()
}
