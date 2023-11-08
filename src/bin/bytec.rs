use simple_interpreter::math_interpreter;

fn main() {
    math_interpreter::interpret_with_bytecode("1+2/3").unwrap();
    math_interpreter::interpret_with_bytecode("(1+1)*(5-3)").unwrap();
}
