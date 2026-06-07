use std::collections::BTreeMap;

use jason::JsonValue;
use jason::to_pretty_string;

#[test]
fn test_nested_array() {
    let json_value = JsonValue::Array(vec![JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Array(vec![
            JsonValue::Number(2.0),
            JsonValue::String("Hi".to_string()),
        ]),
    ])]);
    let expected = "[\n  [\n    1,\n    [\n      2,\n      \"Hi\"\n    ]\n  ]\n]";
    assert_eq!(to_pretty_string(&json_value), expected);
}

#[test]
fn test_nested_object() {
    let mut map = BTreeMap::new();
    let mut inner_map = BTreeMap::new();
    inner_map.insert("b".to_string(), JsonValue::Bool(true));
    let array = vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Object(inner_map),
    ];
    map.insert("a".to_string(), JsonValue::Array(array));
    let obj_json_value = JsonValue::Object(map);
    let expected = "{\n  \"a\": [\n    1,\n    2,\n    {\n      \"b\": true\n    }\n  ]\n}";
    assert_eq!(to_pretty_string(&obj_json_value), expected);
}
