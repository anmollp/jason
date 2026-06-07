use crate::lexer::LexerError;
use crate::parser::ParserError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum JsonError {
    Lexer(LexerError),
    Parser(ParserError),
}

impl std::error::Error for JsonError {}
impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonError::Lexer(err) => write!(f, "{err}"),
            JsonError::Parser(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}
