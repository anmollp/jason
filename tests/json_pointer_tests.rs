use jason::JsonValue;
use std::collections::BTreeMap;

#[test]
fn test_pointer_root() {
    let mut obj = BTreeMap::new();
    obj.insert("name".to_string(), JsonValue::String("John".to_string()));
    obj.insert("age".to_string(), JsonValue::Number(30.0));

    let value = JsonValue::Object(obj);

    // Empty pointer should return the root document
    assert_eq!(value.pointer(""), Some(&value));
}

#[test]
fn test_pointer_object() {
    let mut obj = BTreeMap::new();
    obj.insert("name".to_string(), JsonValue::String("John".to_string()));
    obj.insert("age".to_string(), JsonValue::Number(30.0));

    let value = JsonValue::Object(obj);

    // Access top-level properties
    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("John".to_string()))
    );
    assert_eq!(value.pointer("/age"), Some(&JsonValue::Number(30.0)));

    // Non-existent property returns None
    assert_eq!(value.pointer("/email"), None);
}

#[test]
fn test_pointer_nested_object() {
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

    let value = JsonValue::Object(outer);

    // Access nested properties
    assert_eq!(
        value.pointer("/address/street"),
        Some(&JsonValue::String("123 Main St".to_string()))
    );
    assert_eq!(
        value.pointer("/address/city"),
        Some(&JsonValue::String("Springfield".to_string()))
    );

    // Invalid nested path returns None
    assert_eq!(value.pointer("/address/zip"), None);
}

#[test]
fn test_pointer_array() {
    let array = vec![
        JsonValue::String("first".to_string()),
        JsonValue::String("second".to_string()),
        JsonValue::Number(42.0),
    ];

    let value = JsonValue::Array(array);

    // Access array elements by index
    assert_eq!(
        value.pointer("/0"),
        Some(&JsonValue::String("first".to_string()))
    );
    assert_eq!(
        value.pointer("/1"),
        Some(&JsonValue::String("second".to_string()))
    );
    assert_eq!(value.pointer("/2"), Some(&JsonValue::Number(42.0)));

    // Out of bounds returns None
    assert_eq!(value.pointer("/5"), None);

    // Non-numeric index returns None
    assert_eq!(value.pointer("/invalid"), None);
}

#[test]
fn test_pointer_invalid_path() {
    let mut obj = BTreeMap::new();
    obj.insert("data".to_string(), JsonValue::String("test".to_string()));

    let value = JsonValue::Object(obj);

    // Pointer not starting with '/' returns None
    assert_eq!(value.pointer("data"), None);

    // Accessing array as object returns None
    let array = vec![JsonValue::Number(1.0)];
    let array_value = JsonValue::Array(array);
    assert_eq!(array_value.pointer("/name"), None);

    // Accessing scalar value returns None
    let string_value = JsonValue::String("hello".to_string());
    assert_eq!(string_value.pointer("/0"), None);
}

#[test]
fn test_pointer_escaped_slash() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "data/base".to_string(),
        JsonValue::String("test".to_string()),
    );
    let obj_value = JsonValue::Object(obj);
    assert_eq!(
        obj_value.pointer("/data~1base"),
        Some(&JsonValue::String("test".to_string()))
    );
}

#[test]
fn test_pointer_escaped_tilde() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "data~base".to_string(),
        JsonValue::String("test".to_string()),
    );
    let obj_value = JsonValue::Object(obj);
    assert_eq!(
        obj_value.pointer("/data~0base"),
        Some(&JsonValue::String("test".to_string()))
    );
}

#[test]
fn test_pointer_mut_object() {
    let mut obj = BTreeMap::new();
    obj.insert("name".to_string(), JsonValue::String("John".to_string()));

    let mut value = JsonValue::Object(obj);

    if let Some(JsonValue::String(name)) = value.pointer_mut("/name") {
        *name = "Alice".to_string();
    }

    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("Alice".to_string()))
    );
}

#[test]
fn test_pointer_mut_array() {
    let mut value = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]);

    if let Some(JsonValue::Number(n)) = value.pointer_mut("/1") {
        *n = 42.0;
    }

    assert_eq!(value.pointer("/1"), Some(&JsonValue::Number(42.0)));
}

#[test]
fn test_replace_object_value() {
    let mut obj = BTreeMap::new();
    obj.insert("name".to_string(), JsonValue::String("John".to_string()));

    let replace_value = JsonValue::String("Alice".to_string());

    let mut value = JsonValue::Object(obj);
    value.replace("/name", replace_value);
    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("Alice".to_string()))
    );
}

#[test]
fn test_replace_invalid_path() {
    let mut obj = BTreeMap::new();
    obj.insert("name".to_string(), JsonValue::String("John".to_string()));

    let mut value = JsonValue::Object(obj);

    assert!(!value.replace("/does_not_exist", JsonValue::String("Alice".to_string())));

    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("John".to_string()))
    );
}

#[test]
fn test_replace_array_element() {
    let mut value = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]);

    assert!(value.replace("/1", JsonValue::Number(42.0)));
    assert_eq!(value.pointer("/1"), Some(&JsonValue::Number(42.0)));
}
