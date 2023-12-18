use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use tim::language;

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

        let result = language::interpret(&input);
        if result.is_err() {
            let err = result.unwrap_err();
            println!("{}", err)
        }
    }
}

fn run_file(path: &str) {
    let code = fs::read_to_string(path).expect("Unable to read file");
    let _ = language::interpret(&code).expect("Error running code");
}
