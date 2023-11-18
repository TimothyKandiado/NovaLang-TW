use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    None,
    Number(f64),
    String(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::None => "None".to_string(),
            Self::Number(number) => number.to_string(),
            Self::String(string) => string.clone(),
        };

        write!(f, "{}", description)
    }
}
