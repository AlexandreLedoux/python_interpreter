use std::fmt;
use crate::Value::*;

#[derive(Debug, Clone)]
pub enum Identifier {
    IDENTIFIER,
    KEYWORD
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(u32),
    Str(String)
}

#[derive(Debug, Clone)]
pub struct Token {
    name: Identifier,
    value: Value
}

impl Token {
    pub fn new(name: Identifier, value: Value) -> Self {
        Self { name, value }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Identifier::IDENTIFIER => "Identifier",
            Identifier::KEYWORD => "Keyword"
        })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{}", val),
            Value::Str(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
