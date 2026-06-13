use std::collections::BTreeMap;
use jason::{JsonValue, PatchOperation};

#[test]
fn test_apply_replace_operation() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let op = PatchOperation::Replace {
        path: "/name".to_string(),
        value: JsonValue::String("Alice".to_string()),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("Alice".to_string()))
    );
}

#[test]
fn test_remove_object_value() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let op = PatchOperation::Remove {
        path: "/name".to_string(),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(value.pointer("/name"), None);
}

#[test]
fn test_remove_array_element() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    let op = PatchOperation::Remove {
        path: "/1".to_string(),
    };

    assert!(value.apply(op).is_ok());

    assert_eq!(
        value,
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(3.0),
        ])
    );
}

#[test]
fn test_remove_missing_key() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let op = PatchOperation::Remove {
        path: "/age".to_string(),
    };

    assert!(value.apply(op).is_err());
}

#[test]
fn test_remove_invalid_index() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    let op = PatchOperation::Remove {
        path: "/4".to_string(),
    };

    assert!(value.apply(op).is_err());
}

#[test]
fn test_remove_nexted_object_value() {
    let mut inner = BTreeMap::new();
    inner.insert(
        "street".to_string(),
        JsonValue::String("123 Main St".to_string()),
    );
    inner.insert(
        "city".to_string(),
        JsonValue::String("Springfield".to_string()),
    );

    let mut outer = BTreeMap::new();
    outer.insert("name".to_string(), JsonValue::String("John".to_string()));
    outer.insert("address".to_string(), JsonValue::Object(inner));

    let mut value = JsonValue::Object(outer);

    let op = PatchOperation::Remove {
        path: "/address/city".to_string(),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(value.pointer("/address/city"), None);
}