use jason::JsonValue;
use jason::to_json_string;

#[test]
fn test_serialize_null() {
    assert_eq!(to_json_string(&JsonValue::Null), "null");
}

#[test]
fn test_serialize_bool() {
    assert_eq!(to_json_string(&JsonValue::Bool(true)), "true");
    assert_eq!(to_json_string(&JsonValue::Bool(false)), "false");
}

#[test]
fn test_serialize_number() {
    assert_eq!(to_json_string(&JsonValue::Number(123.0)), "123");
}

#[test]
fn test_serialize_string() {
    assert_eq!(
        to_json_string(&JsonValue::String("quote: \"".to_string())),
        r#""quote: \"""#
    );
    assert_eq!(
        to_json_string(&JsonValue::String("hello".to_string())),
        r#""hello""#
    );
    assert_eq!(
        to_json_string(&JsonValue::String("a\nb".to_string())),
        r#""a\nb""#
    );
    assert_eq!(
        to_json_string(&JsonValue::String("\\".to_string())),
        r#""\\""#
    );
}
