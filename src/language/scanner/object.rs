use std::{
    fmt::Display,
    sync::{Arc, RwLock}, collections::HashMap
};

use interpreter::AstInterpreter;

use crate::language::{abstract_syntax_tree::{interpreter, environment::Environment}, errors, function::FunctionStatement, Token};

pub type WrappedObject = Arc<RwLock<Object>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Callable(Callable),
    Instance(Instance)
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

    pub fn is_class(&self) -> bool {
        match self {
            Self::Callable(Callable::Class(_)) => true,
            _ => false
        }
    } 

    pub fn is_instance(&self) -> bool {
        match self {
            Self::Instance(_) => true,
            _ => false
        }
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
            Self::Instance(instance) => instance.to_string(),
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug, Clone)]
pub enum Callable {
    NativeCall(NativeCall),
    DefinedCall(DefinedCall),
    Class(ClassObject)
}

impl Callable {
    pub fn arity(&self) -> i8 {
        match self {
            Self::NativeCall(native_call) => native_call.arity(),
            Self::DefinedCall(defined_call) => defined_call.arity(),
            Self::Class(class_obj) => class_obj.arity()
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
            Self::Class(class) => class.call(interpreter, arguments)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Class(class) => class.to_string(),
            Self::DefinedCall(defined_call) => defined_call.to_string(),
            Self::NativeCall(native_call) => native_call.to_string(),
        }
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
    pub name: String,
    pub arity: i8,
    pub function: fn(
        interpreter: &mut AstInterpreter,
        arguments: &Vec<WrappedObject>,
    ) -> Result<WrappedObject, errors::Error>,
}

impl NativeCall {
    pub fn new(
        name: String,
        arity: i8,
        function: fn(
            interpreter: &mut AstInterpreter,
            arguments: &Vec<WrappedObject>,
        ) -> Result<WrappedObject, errors::Error>,
    ) -> Self {
        Self { name, arity, function }
    }
    pub fn arity(&self) -> i8 {
        self.arity
    }

    pub fn to_string(&self) -> String {
        format!("function: {}", self.name)
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

    pub fn to_string(&self) -> String {
        format!("function: {}", self.declaration.name.object.to_string())
    }

    pub fn bind(&self, instance: WrappedObject) -> DefinedCall {
        let mut environment = Environment::with_parent(Arc::clone(&self.closure));
        environment.declare_value("this", instance);
        let closure = Arc::new(RwLock::new(environment));

        DefinedCall::new(self.declaration.clone(), closure, self.initializer)
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

#[derive(Debug, Clone)]
pub struct ClassObject {
    name: String,
    superclass: Option<WrappedObject>,
    methods: HashMap<String, WrappedObject>
}

impl ClassObject {
    pub fn new(name: String, superclass: Option<WrappedObject>, methods: HashMap<String, WrappedObject>) -> Self {
        Self { name, superclass, methods }
    }

    pub fn arity(&self) -> i8 {
        if let Some(initializer) = self.methods.get("init") {
            let object_binding = initializer.read();
            if let Object::Callable(Callable::DefinedCall(method)) = &*object_binding.unwrap() {
                return method.arity()
            }
        }

        0
    }

    pub fn find_method(&self, method_name: &str) -> Option<WrappedObject> {
        if let Some(method) = self.methods.get(method_name) {
            return Some(Arc::clone(method));
        }

        if let Some(superclass) = &self.superclass {
            let binding = superclass.read();
            if let Object::Callable(Callable::Class(class)) = &*binding.unwrap() {
                return class.find_method(method_name);
            }
        }

        None
    }

    pub fn call(&self, interpreter: &mut AstInterpreter, arguments: &Vec<WrappedObject>) -> Result<WrappedObject, errors::Error> {
        let instance_id = InstanceID{value: 1};
        let instance = Instance::new(instance_id, self.clone());
        let instance = Object::Instance(instance).wrap();

        let initializer = self.methods.get("init");
        if let Some(initializer) = initializer {
            let initializer = Arc::clone(initializer);
            let binding = initializer.read().unwrap();
            
            if let Object::Callable(Callable::DefinedCall(defined_call)) = &*binding {
                let bound_call = defined_call.bind(Arc::clone(&instance));
                bound_call.call(interpreter, arguments)?;
            }
        }

        Ok(instance)
    }

    pub fn to_string(&self) -> String {
        format!("class: {}", self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct InstanceID {
    pub value: usize
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: InstanceID,
    class: ClassObject,
    fields: HashMap<String, WrappedObject>
}

impl Instance {
    pub fn new(id: InstanceID, class: ClassObject) -> Self {
        Self { id, class, fields: HashMap::new() }
    }

    pub fn get(&self, name_token: &Token) -> Result<WrappedObject, errors::Error> {
        let name = name_token.object.to_string();
        if let Some(field) = self.fields.get(&name) {
            return Ok(Arc::clone(field))
        }

        if let Some(method) = self.class.find_method(&name) {
            return Ok(Arc::clone(&method))
        }

        Err(errors::Error::Runtime(format!("Undefined property {}", name)))
    }

    pub fn set(&mut self, name_token: Token, value: WrappedObject) {
        self.fields.insert(name_token.object.to_string(), value);
    }

    pub fn to_string(&self) -> String {
        return format!("class: {} Instance", self.class.name)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}