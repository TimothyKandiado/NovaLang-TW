use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    None,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::None => false,
            Self::Bool(boolean) => *boolean,
            _ => true
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
        };

        write!(f, "{}", description)
    }
}
