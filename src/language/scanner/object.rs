use std::fmt::Display;

use interpreter::AstInterpreter;

use crate::language::{abstract_syntax_tree::interpreter, errors};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Callable(Callable)
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::None => false,
            Self::Bool(boolean) => *boolean,
            _ => true,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::None => "None".to_string(),
            Self::Number(number) => number.to_string(),
            Self::String(string) => string.clone(),
            Self::Bool(boolean) => boolean.to_string(),
            Self::Callable(_) => "Callable".to_string(),
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Callable {
    NativeCall(NativeCall),
}
 
impl Callable {
    pub fn arity(&self) -> i8 {
        match self {
            Self::NativeCall(native_call) => native_call.arity
        }
    }

    pub fn call(&self, _interpreter: &mut AstInterpreter, arguments: &Vec<Object>) -> Result<Object, errors::Error> {
        match self {
            Self::NativeCall(native_call) => native_call.call(arguments)
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NativeCall {
    arity: i8,
    function: fn(arguments: &Vec<Object>) -> Result<Object, errors::Error>
}

impl NativeCall {
    pub fn arity(&self) -> i8 {
        self.arity
    }

    pub fn call(&self, arguments: &Vec<Object>) -> Result<Object, errors::Error> {
        (self.function)(arguments)
    }
}