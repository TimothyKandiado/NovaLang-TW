#[derive(Debug, Clone)]
pub enum Object {
    None,
    Number(f64),
    String(String),
}

impl Object {

    pub fn to_string(&self) -> String {
        match self {
            Object::String(string) => string.clone(),
            Object::None => "None".to_string(),
            Object::Number(number) => number.to_string(),
        }
    }
}
