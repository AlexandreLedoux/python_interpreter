use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Str(String),
    Int(u32),
    Plus,
    Minus,
    Equal,
    Newline,
    Indent,
    Dedent
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenType,
    line: usize,
    column: usize
}

impl Token {
    pub fn new(kind: TokenType, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}:{}", self.kind, self.line, self.column)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Identifier(val) => write!(f, "Identifier: {}", val),
            TokenType::Str(val) => write!(f, "Str: {}", val),
            TokenType::Int(val) => write!(f, "Int: {}", val),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::Newline => write!(f, "Newline"),
            TokenType::Indent => write!(f, "Indent"),
            TokenType::Dedent => write!(f, "Dedent"),
        }
    }
}

