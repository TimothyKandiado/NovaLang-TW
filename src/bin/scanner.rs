use tim::language;
use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    println!("function of this program is to scan input and return the generated tokens");
    repl()
}

fn repl() {
    loop {
        let mut input = String::new();
        print!("{} ", ">>");
        io::stdout().flush().expect("Error writing to output");
        let input_result = io::stdin().read_line(&mut input);
        if input_result.is_err() {
            eprintln!("Error getting input");
            exit(1)
        }

        let input = input.trim();

        if input == "quit" || input == "Quit" {
            println!("exiting");
            break;
        }

        let result = language::Scanner::new().scan_tokens(&input);

        if let Ok(answer) = result {
            println!("{:?}", answer);
        } else {
            eprintln!("Error: {}", result.unwrap_err())
        }
    }
}