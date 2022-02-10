use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ArgParseError(String);
impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to parse argument action: {}", self.0)
    }
}
impl Error for ArgParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Get,
    Set,
    Insert,
    Remove,
}

impl FromStr for Action {
    type Err = ArgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "get" => Ok(Action::Get),
            "set" => Ok(Action::Set),
            "insert" => Ok(Action::Insert),
            "remove" => Ok(Action::Remove),
            string => Err(ArgParseError(string.to_owned())),
        }
    }
}

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }
}
