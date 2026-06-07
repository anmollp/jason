
use jason::JsonError;
use jason::JsonValue;
use jason::parse_from_str;
use jason::ParserError;
use std::collections::BTreeMap;

// API-level integration test
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

    let mut prefs = BTreeMap::new();
    prefs.insert("theme".into(), JsonValue::String("dark".into()));
    prefs.insert("notifications".into(), JsonValue::String("enabled".into()));

    let mut expected = BTreeMap::new();
    expected.insert("id".into(), JsonValue::Number(1024.0));
    expected.insert("username".into(), JsonValue::String("jdoe_99".into()));
    expected.insert(
        "email".into(),
        JsonValue::String("john.doe@example.com".into()),
    );
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

// Parser valid tests
mod parser_valid_tests {
    use super::*;

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
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_polymorphic_array() -> Result<(), JsonError> {
        let result = parse_from_str("[3, true, \"Hello\"]")?;
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(3.0),
                JsonValue::Bool(true),
                JsonValue::String(String::from("Hello"))
            ])
        );
        Ok(())
    }

    #[test]
    fn test_nested_array() -> Result<(), JsonError> {
        let result = parse_from_str("[1, [2, 3], 4]")?;
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Array(vec![JsonValue::Number(2.0), JsonValue::Number(3.0)]),
                JsonValue::Number(4.0)
            ])
        );
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
        let mut expected = BTreeMap::new();
        expected.insert("key".to_string(), JsonValue::String("value".to_string()));
        assert_eq!(result, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_multi_field_object() -> Result<(), JsonError> {
        let result = parse_from_str(r#"{"a": 1, "b": true}"#)?;
        let mut expected = BTreeMap::new();
        expected.insert("a".to_string(), JsonValue::Number(1.0));
        expected.insert("b".to_string(), JsonValue::Bool(true));
        assert_eq!(result, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_nested_object() -> Result<(), JsonError> {
        let result = parse_from_str(r#"{"a": {"b": 2}}"#)?;
        let mut expected = BTreeMap::new();
        let mut expected_nested = BTreeMap::new();
        expected_nested.insert("b".to_string(), JsonValue::Number(2.0));
        expected.insert("a".to_string(), JsonValue::Object(expected_nested));
        assert_eq!(result, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_empty_object() -> Result<(), JsonError> {
        let result = parse_from_str("{}")?;
        assert_eq!(result, JsonValue::Object(BTreeMap::new()));
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
        let result = parse_from_str(r#""hello\nworld""#)?;
        assert_eq!(result, JsonValue::String("hello\nworld".into()));

        let result = parse_from_str(r#""a\tb\nc""#)?;
        assert_eq!(result, JsonValue::String("a\tb\nc".into()));

        let result = parse_from_str(r#""a\"b""#)?;
        assert_eq!(result, JsonValue::String("a\"b".into()));
        Ok(())
    }

    #[test]
    fn test_whitespace() -> Result<(), JsonError> {
        let result = parse_from_str("\n\t { \n \"a\" : [ 1, 2 ] \n }")?;
        let mut expected = BTreeMap::new();
        expected.insert(
            "a".to_string(),
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
        );
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
        let mut map = BTreeMap::new();
        let mut inner_map = BTreeMap::new();
        inner_map.insert("b".to_string(), JsonValue::Bool(true));
        let array = vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Object(inner_map),
        ];
        map.insert("a".to_string(), JsonValue::Array(array));
        let expected = JsonValue::Object(map);
        assert_eq!(
            parse_from_str(
                r#"
            {
                "a": [1, 2, {"b": true}]
            }"#
            )?,
            expected
        );
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

    #[test]
    fn test_valid_escape_parsing() -> Result<(), JsonError> {
        assert_eq!(
            parse_from_str(r#""hello\nworld""#)?,
            JsonValue::String("hello\nworld".into())
        );

        assert_eq!(
            parse_from_str(r#""a\tb\nc""#)?,
            JsonValue::String("a\tb\nc".into())
        );

        assert_eq!(
            parse_from_str(r#""a\"b""#)?,
            JsonValue::String("a\"b".into())
        );
        Ok(())
    }
}

// Parser invalid tests
mod parser_invalid_tests {
    use super::*;

    fn assert_unexpected_eof(result: Result<JsonValue, JsonError>) {
        match result {
            Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(_))) => {}
            _ => panic!("expected EOF error"),
        }
    }

    fn assert_unexpected_token(result: Result<JsonValue, JsonError>) {
        match result {
            Err(JsonError::Parser(ParserError::UnexpectedToken(_))) => {}
            _ => panic!("expected unexpected token error"),
        }
    }

    fn assert_expected_string_key(result: Result<JsonValue, JsonError>) {
        match result {
            Err(JsonError::Parser(ParserError::ExpectedStringKey(_))) => {}
            _ => panic!("expected string key error"),
        }
    }

    #[test]
    fn test_missing_closing_bracket() {
        let result = parse_from_str("[1, 2");
        match result {
            Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBracket(_))) => {}
            _ => panic!("expected a , or ] error"),
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
            Err(JsonError::Parser(ParserError::TrailingComma(_))) => {}
            _ => panic!("expected trailing comma error"),
        }
        let result = parse_from_str("{\"a\": false,}");
        match result {
            Err(JsonError::Parser(ParserError::TrailingComma(_))) => {}
            _ => panic!("expected trailing comma error"),
        }
    }

    #[test]
    fn test_unexpected_token() {
        let result = parse_from_str("{true: 1}");
        match result {
            Err(JsonError::Parser(ParserError::ExpectedStringKey(_))) => {}
            _ => panic!("expected string key error"),
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
