use jason::JsonError;
use jason::JsonValue;
use jason::LexerError;
use jason::parse_from_str;

fn assert_invalid_literal(result: Result<JsonValue, JsonError>) {
    match result {
        Err(JsonError::Lexer(LexerError::UnexpectedLiteral(_))) => {}
        _ => panic!("Expected a literal true/false/null"),
    }
}

fn assert_invalid_number(result: Result<JsonValue, JsonError>) {
    match result {
        Err(JsonError::Lexer(LexerError::InvalidNumber(_))) => {}
        _ => panic!("expected invalid number error"),
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
    match result {
        Err(JsonError::Lexer(LexerError::UnterminatedString(_))) => {}
        _ => panic!("expected unterminated string error"),
    }
}

#[test]
fn test_invalid_numbers() {
    let result1 = parse_from_str("01");
    match result1 {
        Err(JsonError::Lexer(LexerError::LeadingZero(_))) => {}
        _ => panic!("expected leading zero error"),
    }
    let result2 = parse_from_str("1.");
    assert_invalid_number(result2);
    let result3 = parse_from_str("--1");
    assert_invalid_number(result3);
    let result4 = parse_from_str(".");
    match result4 {
        Err(JsonError::Lexer(LexerError::UnexpectedCharacter { ch: _, position: _ })) => {}
        _ => panic!("expected unexpected character error"),
    }
}

#[test]
fn test_invalid_escaped_string() {
    let result = parse_from_str(r#""\q""#);
    match result {
        Err(JsonError::Lexer(LexerError::InvalidEscapeCharacter { ch: _, position: _ })) => {}
        _ => panic!("expected invalid escape character"),
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
