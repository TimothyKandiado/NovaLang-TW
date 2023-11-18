use tim::language;

fn main() {
    language::interpret_with_bytecode("1+2/3").unwrap();
    language::interpret_with_bytecode("(1+1)*(5-3)").unwrap();
}
