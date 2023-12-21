use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use nova_tw::language::{self, generate_parsed_ast, AstInterpreter};

const PROMPT: &str = ">>";
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        run_file(&args[1])
    } else {
        repl()
    }
}

fn repl() {
    let mut interpreter = AstInterpreter::new();

    loop {
        let mut input = String::new();
        print!("{} ", PROMPT);
        io::stdout().flush().expect("Error writing to output");
        let input_result = io::stdin().read_line(&mut input);
        if input_result.is_err() {
            eprintln!("Error getting input");
            exit(1)
        }

        //let input = input.trim();

        if input == "quit\r\n" || input == "Quit\r\n" {
            println!("exiting");
            break;
        }
        let parsed_ast = generate_parsed_ast(&input);
        if let Err(err) = parsed_ast {
            println!("{}", err);
            continue;
        }
        let parsed_ast = parsed_ast.unwrap();

        let result = interpreter.interpret(parsed_ast);
        //interpreter.print_environment();
        if let Err(err) = result {
            println!("{}", err)
        }
    }
}

fn run_file(path: &str) {
    let result = fs::read_to_string(path);

    if let Err(err) = result {
        println!("{}", err);
        return;
    }

    let code = result.unwrap();

    let result  = language::interpret(&code);

    if let Err(err) = result {
        println!("{}", err);
    }
}
