use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use interpreter::AstInterpreter;

use crate::language::{abstract_syntax_tree::{interpreter, environment::Environment}, errors, function::FunctionStatement};

pub type WrappedObject = Arc<RwLock<Object>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Callable(Callable),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::None => false,
            Self::Bool(boolean) => *boolean,
            _ => true,
        }
    }

    pub fn wrap(self) -> WrappedObject {
        Arc::new(RwLock::new(self))
    }
}



impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::None => "none".to_string(),
            Self::Number(number) => number.to_string(),
            Self::String(string) => string.clone(),
            Self::Bool(boolean) => boolean.to_string(),
            Self::Callable(callable) => callable.to_string(),
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug, Clone)]
pub enum Callable {
    NativeCall(NativeCall),
    DefinedCall(DefinedCall)
}

impl Callable {
    pub fn arity(&self) -> i8 {
        match self {
            Self::NativeCall(native_call) => native_call.arity(),
            Self::DefinedCall(defined_call) => defined_call.arity()
        }
    }

    pub fn call(
        &self,
        interpreter: &mut AstInterpreter,
        arguments: &Vec<WrappedObject>,
    ) -> Result<WrappedObject, errors::Error> {
        match self {
            Self::NativeCall(native_call) => native_call.call(interpreter, arguments),
            Self::DefinedCall(defined_call) => defined_call.call(interpreter, arguments),
        }
    }

    pub fn to_string(&self) -> String {
        "callable".to_string()
    }
}

impl PartialOrd for Callable {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for Callable {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NativeCall {
    pub arity: i8,
    pub function: fn(
        interpreter: &mut AstInterpreter,
        arguments: &Vec<WrappedObject>,
    ) -> Result<WrappedObject, errors::Error>,
}

impl NativeCall {
    pub fn new(
        arity: i8,
        function: fn(
            interpreter: &mut AstInterpreter,
            arguments: &Vec<WrappedObject>,
        ) -> Result<WrappedObject, errors::Error>,
    ) -> Self {
        Self { arity, function }
    }
    pub fn arity(&self) -> i8 {
        self.arity
    }

    pub fn call(
        &self,
        interpreter: &mut AstInterpreter,
        arguments: &Vec<WrappedObject>,
    ) -> Result<WrappedObject, errors::Error> {
        (self.function)(interpreter, arguments)
    }
}

#[derive(Debug, Clone)]
pub struct DefinedCall {
    declaration: Box<FunctionStatement>,
    closure: Arc<RwLock<Environment>>,
    pub initializer: bool
}

impl DefinedCall {
    pub fn new(declaration: Box<FunctionStatement>, closure: Arc<RwLock<Environment>>, initializer: bool) -> Self {
        Self {
            declaration, closure, initializer
        }
    }
    pub fn arity(&self) -> i8 {
        self.declaration.parameters.len() as i8
    }

    pub fn call(
        &self,
        interpreter: &mut AstInterpreter,
        arguments: &Vec<WrappedObject>,
    ) -> Result<WrappedObject, errors::Error> {
        let mut environment = Environment::with_parent(Arc::clone(&self.closure));
        for (parameter, value) in &self.declaration.parameters.iter().zip(arguments).collect::<Vec<_>>() {
            environment.declare_value(parameter.object.to_string().as_str(), Arc::clone(*value));
        }

        let new_environment = Arc::new(RwLock::new(environment));


        let result = interpreter.execute_block(&self.declaration.body, new_environment);

        if result.is_ok() {
            return Ok(Object::None.wrap())
        }

        let error = result.unwrap_err();
        if let errors::Error::Return(object) = error {
            return Ok(object)
        }
        
        Err(error)
    }
}
