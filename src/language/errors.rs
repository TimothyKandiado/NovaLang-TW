use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use super::Object;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Scan(String),
    Interpret(String),
    Runtime(String),
    Return(Arc<RwLock<Object>>),
    Exit(usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Parse(description) => description.clone(),
                Self::Scan(description) => description.clone(),
                Self::Interpret(description) => description.clone(),
                Self::Runtime(description) => description.clone(),
                Self::Return(_) => "return".to_string(),
                Self::Exit(code) => format!("{}", code),
            }
        )
    }
}

impl Error {
    pub fn is_exit(&self) -> bool {
        matches!(self, Self::Exit(_))
    }
    
    pub fn scan_error(description: &str) -> Self {
        Self::Scan(description.to_string())
    }

    pub fn parse_error(description: &str) -> Self {
        Self::Parse(description.to_string())
    }

    pub fn intepret_error(description: &str) -> Self {
        Self::Interpret(description.to_string())
    }
}
