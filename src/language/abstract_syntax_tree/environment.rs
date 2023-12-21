use std::borrow::BorrowMut;
use std::fmt::Display;
use std::sync::RwLock;
use std::{collections::HashMap as Map, sync::Arc};

use crate::language::{errors, scanner::object::Object};

#[derive(Debug)]
pub struct Environment {
    parent: Option<Arc<RwLock<Environment>>>,
    values: Map<String, Arc<RwLock<Object>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: Map::new(),
        }
    }

    pub fn with_parent(parent: Arc<RwLock<Environment>>) -> Self {
        Self {
            parent: Some(parent),
            values: Map::new(),
        }
    }

    pub fn declare_value(&mut self, name: &str, value: Arc<RwLock<Object>>) {
        self.values.insert(name.to_string(), value);
    }

    pub fn set_value(
        &mut self,
        name: &str,
        value: Arc<RwLock<Object>>,
    ) -> Result<(), errors::Error> {
        if self.values.contains_key(name) {
            self.declare_value(name, value);
            return Ok(());
        }

        if let Some(parent) = &self.parent {
            let parent = Arc::clone(parent);
            let parent_writer = parent.write();
            if let Ok(mut parent_writer) = parent_writer {
                return (*parent_writer).set_value(name, value);
            }

            return Err(errors::Error::Runtime(
                parent_writer.unwrap_err().to_string(),
            ));
        }

        Err(errors::Error::Runtime(
            "Can not set a variable that was not declared".to_string(),
        ))
    }

    pub fn get_value(&self, name: &str) -> Arc<RwLock<Object>> {
        if let Some(value) = self.values.get(name) {
            return Arc::clone(value);
        }

        if let Some(parent) = &self.parent {
            let parent = Arc::clone(parent);
            let parent_reader = parent.read();

            if let Ok(parent_reader) = parent_reader {
                return (*parent_reader).get_value(name);
            }
        }

        Arc::new(RwLock::new(Object::None))
    }

    #[allow(dead_code)]
    pub fn delete_value(&mut self, name: &str) {
        if self.values.contains_key(name) {
            self.values.borrow_mut().remove(name);
            return;
        }

        if let Some(parent) = &self.parent {
            let parent = Arc::clone(parent);
            let parent_writer = parent.write();

            if let Ok(mut parent_writer) = parent_writer {
                (*parent_writer).delete_value(name)
            }
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parent_env = String::new();
        if let Some(parent) = &self.parent {
            if let Ok(parent) = parent.read() {
                parent_env = parent.to_string();
            }
        }

        let mut values = String::new();

        for (name, value) in &self.values {
            if let Ok(value) = value.read() {
                values.push_str(&format!(" [id: {} => value : {}]", name, (*value)));
                values.push('\n');
            }
        }

        write!(f, "{}\n[\n{}]", parent_env, values)
    }
}
