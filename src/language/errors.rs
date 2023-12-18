use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    ScanError(String),
    InterpretError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ParseError(description) => description,
            Self::ScanError(description) => description,
            Self::InterpretError(description) => description
        })
    }
}

impl Error {
    pub fn scan_error(description: &str) -> Self {
        Self::ScanError(description.to_string())
    }

    pub fn parse_error(description: &str) -> Self {
        Self::ParseError(description.to_string())
    }

    pub fn intepret_error(description: &str) -> Self {
        Self::InterpretError(description.to_string())
    }
}