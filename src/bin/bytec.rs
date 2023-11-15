use simple_interpreter::interpreter;

fn main() {
    interpreter::interpret_with_bytecode("1+2/3").unwrap();
    interpreter::interpret_with_bytecode("(1+1)*(5-3)").unwrap();
}
