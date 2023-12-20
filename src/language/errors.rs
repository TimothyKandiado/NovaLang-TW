use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Scan(String),
    Interpret(String),
    Runtime(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Parse(description) => description,
                Self::Scan(description) => description,
                Self::Interpret(description) => description,
                Self::Runtime(description) => description,
            }
        )
    }
}

impl Error {
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
