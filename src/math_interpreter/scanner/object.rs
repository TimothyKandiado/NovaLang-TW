#[derive(Debug, Clone)]
pub enum Object {
    None,
    Number(f64),
    String(String),
}

impl Object {
    pub fn is_none(&self) -> bool {
        match self {
            Object::None => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Object::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Object::String(_) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Object::String(string) => string.clone(),
            Object::None => "None".to_string(),
            Object::Number(number) => number.to_string(),
        }
    }
}
