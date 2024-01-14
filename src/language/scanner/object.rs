use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, RwLock},
};

use interpreter::AstInterpreter;

use crate::language::{
    abstract_syntax_tree::{environment::Environment, interpreter},
    errors,
    function::FunctionStatement,
    Token,
};

pub type WrappedObject = Arc<RwLock<Object>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Callable(Callable),
    Instance(Instance),
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
        matches!(self, Self::Callable(Callable::Class(_)))
    }

    pub fn is_instance(&self) -> bool {
        matches!(self, Self::Instance(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// returns true if object can be copied to create another object,
    /// returns false if object should be referenced instead
    pub fn prefers_copy(&self) -> bool {
        matches!(self, Self::Bool(_) | Self::Number(_) | Self::String(_))
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
    Class(ClassObject),
}

impl Callable {
    pub fn arity(&self) -> i8 {
        match self {
            Self::NativeCall(native_call) => native_call.arity(),
            Self::DefinedCall(defined_call) => defined_call.arity(),
            Self::Class(class_obj) => class_obj.arity(),
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
            Self::Class(class) => class.call(interpreter, arguments),
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

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Class(class) => class.to_string(),
                Self::DefinedCall(defined_call) => defined_call.to_string(),
                Self::NativeCall(native_call) => native_call.to_string(),
            }
        )
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
        Self {
            name,
            arity,
            function,
        }
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

impl Display for NativeCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function: {}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct DefinedCall {
    declaration: Box<FunctionStatement>,
    closure: Arc<RwLock<Environment>>,
    pub initializer: bool,
}

impl DefinedCall {
    pub fn new(
        declaration: Box<FunctionStatement>,
        closure: Arc<RwLock<Environment>>,
        initializer: bool,
    ) -> Self {
        Self {
            declaration,
            closure,
            initializer,
        }
    }
    pub fn arity(&self) -> i8 {
        self.declaration.parameters.len() as i8
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
        for (parameter, value) in &self
            .declaration
            .parameters
            .iter()
            .zip(arguments)
            .collect::<Vec<_>>()
        {
            environment.declare_value(parameter.object.to_string().as_str(), Arc::clone(*value));
        }

        let new_environment = Arc::new(RwLock::new(environment));

        let result = interpreter.execute_block(&self.declaration.body, new_environment);

        if result.is_ok() {
            return Ok(Object::None.wrap());
        }

        let error = result.unwrap_err();
        if let errors::Error::Return(object) = error {
            return Ok(object);
        }

        Err(error)
    }
}

impl Display for DefinedCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "function: {}", self.declaration.name.object)
    }
}

#[derive(Debug, Clone)]
pub struct ClassObject {
    name: String,
    superclass: Option<WrappedObject>,
    methods: HashMap<String, WrappedObject>,
}

impl ClassObject {
    pub fn new(
        name: String,
        superclass: Option<WrappedObject>,
        methods: HashMap<String, WrappedObject>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub fn arity(&self) -> i8 {
        if let Some(initializer) = self.methods.get("init") {
            let object_binding = initializer.read();
            if let Object::Callable(Callable::DefinedCall(method)) = &*object_binding.unwrap() {
                return method.arity();
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

    pub fn call(
        &self,
        interpreter: &mut AstInterpreter,
        arguments: &Vec<WrappedObject>,
    ) -> Result<WrappedObject, errors::Error> {
        let instance_id = interpreter.id_maker.get_new_id();
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
}

impl Display for ClassObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class : {}", self.name)
    }
}

#[derive(Debug, Default)]
pub struct InstanceIDCreator {
    current_id: u128,
}

impl InstanceIDCreator {
    pub fn new() -> Self {
        Self { current_id: 0 }
    }

    pub fn get_new_id(&mut self) -> InstanceID {
        let id = InstanceID {
            value: self.current_id,
        };
        self.current_id += 1;
        id
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct InstanceID {
    pub value: u128,
}

impl Display for InstanceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: InstanceID,
    class: ClassObject,
    fields: HashMap<String, WrappedObject>,
}

impl Instance {
    pub fn new(id: InstanceID, class: ClassObject) -> Self {
        Self {
            id,
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name_token: &Token) -> Result<WrappedObject, errors::Error> {
        let name = name_token.object.to_string();
        if let Some(field) = self.fields.get(&name) {
            return Ok(Arc::clone(field));
        }

        if let Some(method) = self.class.find_method(&name) {
            return Ok(Arc::clone(&method));
        }

        Err(errors::Error::Runtime(format!(
            "Undefined property {}",
            name
        )))
    }

    pub fn set(&mut self, name_token: Token, value: WrappedObject) {
        #[cfg(feature = "debug")]
        println!(
            "(dbg) setting field {} = {}",
            name_token.object.to_string(),
            value.read().unwrap().to_string()
        );
        self.fields.insert(name_token.object.to_string(), value);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description = String::new();
        description.push_str(&format!(
            "Instance of {} | ID: {}",
            self.class.name, &self.id
        ));
        for (name, field) in &self.fields {
            description.push('\n');
            let binding = field.read().unwrap();
            description.push_str(&format!("field: ({} = {})", name, (*binding)));
        }

        write!(f, "{}", description)
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
