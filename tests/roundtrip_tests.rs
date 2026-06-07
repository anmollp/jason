use jason::JsonError;
use jason::parse_from_str;
use jason::to_json_string;

#[test]
fn test_round_trip_simple_object() -> Result<(), JsonError> {
    let original = r#"{"a":1,"b":true}"#;
    let value = parse_from_str(original)?;
    let serialized = to_json_string(&value);
    let deserialized = parse_from_str(&serialized)?;

    assert_eq!(value, deserialized);
    Ok(())
}

#[test]
fn test_round_trip_complex_object() -> Result<(), JsonError> {
    let original = r#"{"a":[1,2,3,{"b":"hello\nworld"}],"c":true}"#;
    let value = parse_from_str(original)?;
    let serialized = to_json_string(&value);
    let deserialized = parse_from_str(&serialized)?;

    assert_eq!(value, deserialized);
    Ok(())
}

#[test]
fn test_round_trip_edge_whitespace_handling() -> Result<(), JsonError> {
    let original = r#"" { \"a\" : 1 } ""#;
    let value = parse_from_str(original)?;
    let serialized = to_json_string(&value);
    let deserialized = parse_from_str(&serialized)?;

    assert_eq!(value, deserialized);
    Ok(())
}

#[test]
fn test_round_trip_escape_heavy_string() -> Result<(), JsonError> {
    let original = r#"{"s":"\"\\\n\t"}"#;
    let value = parse_from_str(original)?;
    let serialized = to_json_string(&value);
    let deserialized = parse_from_str(&serialized)?;

    assert_eq!(value, deserialized);
    Ok(())
}

#[test]
fn test_round_trip_deep_nesting() -> Result<(), JsonError> {
    let original = r#"{"a":[{"b":[{"c":[1,2,3]}]}]}"#;
    let value = parse_from_str(original)?;
    let serialized = to_json_string(&value);
    let deserialized = parse_from_str(&serialized)?;

    assert_eq!(value, deserialized);
    Ok(())
}

#[test]
fn test_round_trip_empt_structure() -> Result<(), JsonError> {
    let original = r#"[]"#;
    let value = parse_from_str(original)?;
    let serialized = to_json_string(&value);
    let deserialized = parse_from_str(&serialized)?;

    assert_eq!(value, deserialized);
    Ok(())
}
