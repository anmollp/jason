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

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: usize,
    column: usize,
}

#[cfg(test)]
mod tests {

    mod lexer_invalid_tests {
        use crate::{parse_from_str, JsonError, JsonValue };
        use crate::lexer::LexerError;

        fn assert_invalid_literal(result: Result<JsonValue, JsonError>) {
            match result {
                Err(JsonError::Lexer(LexerError::UnexpectedLiteral(_))) => {},
                _ => panic!("Expected a literal true/false/null")
            }
        }

        fn assert_invalid_number(result: Result<JsonValue, JsonError>) {
            match result {
                Err(JsonError::Lexer(LexerError::InvalidNumber(_))) => {},
                _ => panic!("expected invalid number error")
            }
        }

        #[test]
        fn test_literal_parse() {
            let result = parse_from_str("truex");
            assert_invalid_literal(result)
        }

        #[test]
        fn test_unterminated_string() {
            let result = parse_from_str("\"olleh");
            println!("{:?}", result);
            match result {
                Err(JsonError::Lexer(LexerError::UnterminatedString(_))) => {},
                _ => panic!("expected unterminated string error")
            }
        }

        #[test]
        fn test_invalid_numbers() {
            let result1 = parse_from_str("01");
            match result1 {
                Err(JsonError::Lexer(LexerError::LeadingZero(_))) => {},
                _ => panic!("expected leading zero error")
            }
            let result2 = parse_from_str("1.");
            assert_invalid_number(result2);
            let result3 = parse_from_str("--1");
            assert_invalid_number(result3);
            let result4 = parse_from_str(".");
            match result4 {
                Err(JsonError::Lexer(LexerError::UnexpectedCharacter { ch: _, position: _ })) => {},
                _ => panic!("expected unexpected character error")
            }
        }

        #[test]
        fn test_invalid_escaped_string() {
            let result = parse_from_str(r#""\q""#);
            match result {
                Err(JsonError::Lexer(LexerError::InvalidEscapeCharacter{ ch: _, position: _})) => {},
                _ => panic!("expected invalid escape character")
            }
        }

        #[test]
        fn test_position_tracking() {
            let result = parse_from_str("0001");
            match result {
                Err(JsonError::Lexer(LexerError::LeadingZero(pos))) => {
                    assert_eq!(pos.line, 1);
                    assert_eq!(pos.column, 2);
                }
                _ => panic!("expected Leading zero error"),
            }
        }

        #[test]
        fn test_invalid_exponent_numbers() {
            let result = parse_from_str("1e+");
            assert_invalid_number(result);
            let result = parse_from_str("1ee10");
            assert_invalid_number(result);
            let result = parse_from_str("1e-+3");
            assert_invalid_number(result);
        }
    }

    mod parser_valid_tests {
        use std::collections::HashMap;
        use crate::{parse_from_str, JsonError, JsonValue};

        #[test]
        fn test_parse_integer() -> Result<(), JsonError> {
            let result = parse_from_str("123")?;
            assert_eq!(result, JsonValue::Number(123.0));
            Ok(())
        }

        #[test]
        fn test_parse_float() -> Result<(), JsonError> {
            let result = parse_from_str("-1.0")?;
            assert_eq!(result, JsonValue::Number(-1.0));
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
        fn test_empty_array() -> Result<(), JsonError> {
            let result = parse_from_str("[]")?;
            assert_eq!(result, JsonValue::Array(vec![]));
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

        #[test]
        fn test_escaped_string() -> Result<(), JsonError> {
            let result = parse_from_str(r#""quote: \"""#)?;
            assert_eq!(result, JsonValue::String("quote: \"".to_string()));
            let result = parse_from_str(r#""\\""#)?;
            assert_eq!(result, JsonValue::String("\\".to_string()));
            let result = parse_from_str(r#""\n""#)?;
            assert_eq!(result, JsonValue::String("\n".to_string()));
            let result = parse_from_str(r#""\t""#)?;
            assert_eq!(result, JsonValue::String("\t".to_string()));
            Ok(())
        }

        #[test]
        fn test_whitespace() -> Result<(), JsonError> {
            let result = parse_from_str("\n\t { \n \"a\" : [ 1, 2 ] \n }")?;
            let mut expected = HashMap::new();
            expected.insert("a".to_string(), JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]));
            assert_eq!(result, JsonValue::Object(expected));
            Ok(())
        }

        #[test]
        fn test_valid_numbers() -> Result<(), JsonError> {
            let result1 = parse_from_str("0")?;
            assert_eq!(result1, JsonValue::Number(0.0));
            let result2 = parse_from_str("-0")?;
            assert_eq!(result2, JsonValue::Number(-0.0));
            let result3 = parse_from_str("0.5")?;
            assert_eq!(result3, JsonValue::Number(0.5));
            Ok(())
        }

        #[test]
        fn test_root_values() -> Result<(), JsonError> {
            assert_eq!(parse_from_str("true")?, JsonValue::Bool(true));
            assert_eq!(parse_from_str("false")?, JsonValue::Bool(false));
            assert_eq!(parse_from_str("null")?, JsonValue::Null);
            assert_eq!(
                parse_from_str(r#""hello""#)?,
                JsonValue::String("hello".into())
            );
            Ok(())
        }

        #[test]
        fn test_mixed_nesting() -> Result<(), JsonError> {
            let mut map = HashMap::new();
            let mut inner_map = HashMap::new();
            inner_map.insert("b".to_string(), JsonValue::Bool(true));
            let array = vec![JsonValue::Number(1.0), JsonValue::Number(2.0), JsonValue::Object(inner_map)];
            map.insert("a".to_string(), JsonValue::Array(array));
            let expected = JsonValue::Object(map);
            assert_eq!(parse_from_str(r#"
            {
                "a": [1, 2, {"b": true}]
            }"#)?,
            expected);
            Ok(())
        }

        #[test]
        fn test_valid_exponent_numbers() -> Result<(), JsonError> {
            assert_eq!(parse_from_str("1e10")?, JsonValue::Number(10000000000.0));
            assert_eq!(parse_from_str("1E10")?, JsonValue::Number(10000000000.0));
            assert_eq!(parse_from_str("1.5e-3")?, JsonValue::Number(0.0015));
            assert_eq!(parse_from_str("1.5e+3")?, JsonValue::Number(1500.0));
            assert_eq!(parse_from_str("-1.5e3")?, JsonValue::Number(-1500.0));
            Ok(())
        }
    }

    mod parser_invalid_tests {
        use crate::{parse_from_str, JsonError, JsonValue};
        use crate::parser::ParserError;

        fn assert_unexpected_eof(result: Result<JsonValue, JsonError>) {
            match result {
                Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(_))) => {},
                _ => panic!("expected EOF error")
            }
        }

        fn assert_unexpected_token(result: Result<JsonValue, JsonError>) {
            match result {
                Err(JsonError::Parser(ParserError::UnexpectedToken(_))) => {},
                _ => panic!("expected unexpected token error")
            }
        }

        fn assert_expected_string_key(result: Result<JsonValue, JsonError>) {
            match result {
                Err(JsonError::Parser(ParserError::ExpectedStringKey(_))) => {},
                _ => panic!("expected string key error")
            }
        }

        #[test]
        fn test_missing_closing_bracket() {
            let result = parse_from_str("[1, 2");
            println!("{:?}", result);
            match result {
                Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBracket(_))) => {},
                _ => panic!("expected a , or ] error")
            }
        }

        #[test]
        fn test_missing_colon() {
            let result = parse_from_str("{\"a\" 1}");
            assert_unexpected_token(result)
        }

        #[test]
        fn test_trailing_comma() {
            let result = parse_from_str("[1, 2,]");
            match result {
                Err(JsonError::Parser(ParserError::TrailingComma(_))) => {},
                _ => panic!("expected trailing comma error")
            }
            let result = parse_from_str("{\"a\": false,}");
            match result {
                Err(JsonError::Parser(ParserError::TrailingComma(_))) => {},
                _ => panic!("expected trailing comma error")
            }
        }

        #[test]
        fn test_unexpected_token() {
            let result = parse_from_str("{true: 1}");
            match result {
                Err(JsonError::Parser(ParserError::ExpectedStringKey(_))) => {},
                _ => panic!("expected string key error")
            }
        }

        #[test]
        fn test_parse_invalid_number() {
            let result = parse_from_str("-3.1.4");
            assert!(result.is_err());
        }

        #[test]
        fn test_unterminated_array() {
            let result = parse_from_str("[");
            assert_unexpected_eof(result)
        }

        #[test]
        fn test_eof_after_comma() {
            let result = parse_from_str("[1,");
            assert_unexpected_eof(result)
        }

        #[test]
        fn test_missing_object_value() {
            let result = parse_from_str(r#"{"a":"#);
            assert_unexpected_eof(result)
        }

        #[test]
        fn test_invalid_array_structure() {
            let result = parse_from_str("[,]");
            assert_unexpected_token(result)
        }

        #[test]
        fn test_invalid_object_structure() {
            let result1 = parse_from_str("{\"a\":}");
            assert_unexpected_token(result1);
            let result2 = parse_from_str("{,}");
            assert_expected_string_key(result2);
            let result3 = parse_from_str("{\"a\",1}");
            assert_unexpected_token(result3)
        }

        #[test]
        fn test_invalid_extra_tokens() {
            let result = parse_from_str("true false");
            assert_unexpected_token(result);
            let result = parse_from_str("[1] [2]");
            assert_unexpected_token(result);
            let result = parse_from_str("{} {}");
            assert_unexpected_token(result);
        }

        #[test]
        fn test_position_tracking() {
            let result = parse_from_str("{\ntrue: 1}");
            match result {
                Err(JsonError::Parser(ParserError::ExpectedStringKey(pos))) => {
                    assert_eq!(pos.line, 2);
                    assert_eq!(pos.column, 1);
                }
                _ => panic!("expected ExpectedStringKey"),
            }
        }

        #[test]
        fn test_multiline_position_tracking() {
            let result = parse_from_str("{\ntrue: 1}");
            match result {
                Err(JsonError::Parser(ParserError::ExpectedStringKey(pos))) => {
                    assert_eq!(pos.line, 2);
                    assert_eq!(pos.column, 1);
                }
                _ => panic!("expected ExpectedStringKey error"),
            }
        }
    }

    mod integration_tests {
        use std::collections::HashMap;
        use crate::{parse_from_str, JsonError, JsonValue };

        #[test]
        fn test_parse_json() -> Result<(), JsonError> {
            let example_json = r#"{
              "id": 1024,
              "username": "jdoe_99",
              "email": "john.doe@example.com",
              "isActive": true ,
              "roles": ["Admin", "Editor"],
              "preferences": {
                "theme": "dark",
                "notifications": "enabled"
              },
              "loginCount": 42,
              "lastLogin": null
            }"#;

            let result = parse_from_str(example_json)?;

            let mut prefs = HashMap::new();
            prefs.insert("theme".into(), JsonValue::String("dark".into()));
            prefs.insert("notifications".into(), JsonValue::String("enabled".into()));

            let mut expected = HashMap::new();
            expected.insert("id".into(), JsonValue::Number(1024.0));
            expected.insert("username".into(), JsonValue::String("jdoe_99".into()));
            expected.insert("email".into(), JsonValue::String("john.doe@example.com".into()));
            expected.insert("isActive".into(), JsonValue::Bool(true));
            expected.insert(
                "roles".into(),
                JsonValue::Array(vec![
                    JsonValue::String("Admin".into()),
                    JsonValue::String("Editor".into()),
                ]),
            );
            expected.insert("preferences".into(), JsonValue::Object(prefs));
            expected.insert("loginCount".into(), JsonValue::Number(42.0));
            expected.insert("lastLogin".into(), JsonValue::Null);

            assert_eq!(result, JsonValue::Object(expected));
            Ok(())
        }
    }
}