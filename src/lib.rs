pub mod lexer;
pub mod parser;

use std::collections::HashMap;

pub use parser::parse_from_str;
use crate::lexer::LexerError;
use crate::parser::ParserError;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

#[derive(Debug)]
pub enum JsonError {
    Lexer(LexerError),
    Parser(ParserError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() -> Result<(), JsonError> {
        let result = parse_from_str("123")?;
        assert_eq!(result, JsonValue::Number(123.0));
        Ok(())
    }

    #[test]
    fn test_parse_array() -> Result<(), JsonError> {
        let result = parse_from_str("[1, 2, 3]")?;
        assert_eq!(result, JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]
        ));
        Ok(())
    }

    #[test]
    fn test_polymorphic_array() -> Result<(), JsonError> {
        let result = parse_from_str("[3, true, \"Hello\"]")?;
        assert_eq!(result, JsonValue::Array(vec![
            JsonValue::Number(3.0),
            JsonValue::Bool(true),
            JsonValue::String(String::from("Hello"))
        ]));
        Ok(())
    }

    #[test]
    fn test_nested_array() -> Result<(), JsonError> {
        let result = parse_from_str("[1, [2, 3], 4]")?;
        assert_eq!(result, JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Array(vec![
                JsonValue::Number(2.0),
                JsonValue::Number(3.0)
            ]),
            JsonValue::Number(4.0)
        ]));
        Ok(())
    }

    #[test]
    fn test_simple_object() -> Result<(), JsonError> {
        let result = parse_from_str("{\"key\": \"value\"}")?;
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), JsonValue::String("value".to_string()));
        assert_eq!(result, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_multi_field_object() -> Result<(), JsonError> {
        let result = parse_from_str(r#"{"a": 1, "b": true}"#)?;
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), JsonValue::Number(1.0));
        expected.insert("b".to_string(), JsonValue::Bool(true));
        assert_eq!(result, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_nested_object() -> Result<(), JsonError> {
        let result = parse_from_str(r#"{"a": {"b": 2}}"#)?;
        let mut expected = HashMap::new();
        let mut expected_nested = HashMap::new();
        expected_nested.insert("b".to_string(), JsonValue::Number(2.0));
        expected.insert("a".to_string(), JsonValue::Object(expected_nested));
        assert_eq!(result, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_missing_closing_bracket() {
        let result = parse_from_str("[1, 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_colon() {
        let result = parse_from_str("{\"a\" 1}");
        assert!(result.is_err());
    }

    #[test]
    fn test_trailing_comma() {
        let result = parse_from_str("[1, 2,]");
        assert!(result.is_err());
        let result = parse_from_str("{\"a\": false,}");
        assert!(result.is_err());
    }

    #[test]
    fn test_unterminated_string() {
        let result = parse_from_str("\"olleh");
        assert!(result.is_err());
    }

    #[test]
    fn test_unexpected_token() {
        let result = parse_from_str("{true: 1}");
        assert!(result.is_err());
    }

    #[test]
    fn test_escaped_string() -> Result<(), JsonError> {
        let result = parse_from_str(r#""quote: \"""#)?;
        assert_eq!(result, JsonValue::String("quote: \"".to_string()));
        Ok(())
    }

    #[test]
    fn test_parse_float() -> Result<(), JsonError> {
        let result = parse_from_str("-1.0")?;
        assert_eq!(result, JsonValue::Number(-1.0));
        Ok(())
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = parse_from_str("-3.1.4");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_array() -> Result<(), JsonError> {
        let result = parse_from_str("[]")?;
        assert_eq!(result, JsonValue::Array(vec![]));
        Ok(())
    }

    #[test]
    fn test_empty_object() -> Result<(), JsonError> {
        let result = parse_from_str("{}")?;
        assert_eq!(result, JsonValue::Object(HashMap::new()));
        Ok(())
    }

    #[test]
    fn test_unicode_parse() -> Result<(), JsonError> {
        let result = parse_from_str(r#""\u0041\u0050""#)?;
        assert_eq!(result, JsonValue::String("AP".to_string()));
        Ok(())
    }
}