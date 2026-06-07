mod error;
mod lexer;
mod parser;
mod serializer;
mod value;

use crate::lexer::Lexer;
use crate::parser::Parser;
pub use lexer::LexerError;
pub use parser::ParserError;
pub use error::JsonError;
pub use value::JsonValue;

pub fn parse_from_str(input: &str) -> Result<JsonValue, JsonError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    parser.parse()
}

pub fn to_json_string(value: &JsonValue) -> String {
    let mut output = String::new();
    serializer::serialize(value, &mut output);
    output
}

pub fn to_pretty_string(value: &JsonValue) -> String {
    let mut output = String::new();
    serializer::pretty_serialize(value, &mut output, 0);
    output
}
