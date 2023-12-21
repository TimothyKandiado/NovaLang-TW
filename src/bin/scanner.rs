use nova_tw::language;
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        scan_file(&args[1])
    } else {
        println!("function of this program is to scan input and return the generated tokens");
        repl()
    }
}

fn repl() {
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().expect("Error writing to output");
        let input_result = io::stdin().read_line(&mut input);
        if input_result.is_err() {
            eprintln!("Error getting input");
            exit(1)
        }

        println!("input: {:?}", input);
        //let input = input.trim();

        if input == "quit\r\n" || input == "Quit\r\n" {
            println!("exiting");
            break;
        }

        let result = language::Scanner::new().scan_tokens(&input);

        if let Ok(answer) = result {
            language::debug_print_tokens(answer);
        } else {
            eprintln!("Error: {}", result.unwrap_err())
        }
    }
}

fn scan_file(path: &str) {
    let code = fs::read_to_string(path).expect("Cannot read file");
    let tokens = language::Scanner::new()
        .scan_tokens(&code)
        .expect("Could not scan tokens");

    language::debug_print_tokens(tokens);
}
