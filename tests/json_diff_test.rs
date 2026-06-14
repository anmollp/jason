use std::collections::BTreeMap;
use jason::{diff, JsonValue};

#[test]
fn test_diff_apply_round_trip() {
    let mut old_map = BTreeMap::new();
    old_map.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );
    old_map.insert(
        "age".to_string(),
        JsonValue::Number(30.0),
    );
    let old = JsonValue::Object(old_map);

    let mut new_map = BTreeMap::new();
    new_map.insert(
        "name".to_string(),
        JsonValue::String("Jane".to_string()),
    );
    new_map.insert(
        "email".to_string(),
        JsonValue::String("jane@example.com".to_string()),
    );
    let new = JsonValue::Object(new_map);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}

#[test]
fn test_diff_array_modifications() {
    let old = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    let new = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
        JsonValue::Number(4.0),
    ]);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}

#[test]
fn test_diff_nested_object_changes() {
    let mut old_inner = BTreeMap::new();
    old_inner.insert(
        "city".to_string(),
        JsonValue::String("New York".to_string()),
    );

    let mut old_map = BTreeMap::new();
    old_map.insert("name".to_string(), JsonValue::String("John".to_string()));
    old_map.insert("address".to_string(), JsonValue::Object(old_inner));
    let old = JsonValue::Object(old_map);

    let mut new_inner = BTreeMap::new();
    new_inner.insert(
        "city".to_string(),
        JsonValue::String("Los Angeles".to_string()),
    );
    new_inner.insert(
        "zip".to_string(),
        JsonValue::String("90001".to_string()),
    );

    let mut new_map = BTreeMap::new();
    new_map.insert("name".to_string(), JsonValue::String("John".to_string()));
    new_map.insert("address".to_string(), JsonValue::Object(new_inner));
    let new = JsonValue::Object(new_map);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}

#[test]
fn test_diff_array_of_objects() {
    let mut old_obj1 = BTreeMap::new();
    old_obj1.insert("id".to_string(), JsonValue::Number(1.0));
    old_obj1.insert("name".to_string(), JsonValue::String("Alice".to_string()));

    let mut old_obj2 = BTreeMap::new();
    old_obj2.insert("id".to_string(), JsonValue::Number(2.0));
    old_obj2.insert("name".to_string(), JsonValue::String("Bob".to_string()));

    let old = JsonValue::Array(vec![
        JsonValue::Object(old_obj1),
        JsonValue::Object(old_obj2),
    ]);

    let mut new_obj1 = BTreeMap::new();
    new_obj1.insert("id".to_string(), JsonValue::Number(1.0));
    new_obj1.insert("name".to_string(), JsonValue::String("Alice".to_string()));
    new_obj1.insert("status".to_string(), JsonValue::String("active".to_string()));

    let mut new_obj2 = BTreeMap::new();
    new_obj2.insert("id".to_string(), JsonValue::Number(2.0));
    new_obj2.insert("name".to_string(), JsonValue::String("Bob".to_string()));

    let new = JsonValue::Array(vec![
        JsonValue::Object(new_obj1),
        JsonValue::Object(new_obj2),
    ]);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}

#[test]
fn test_diff_type_changes() {
    let mut old_map = BTreeMap::new();
    old_map.insert("value".to_string(), JsonValue::String("123".to_string()));
    let old = JsonValue::Object(old_map);

    let mut new_map = BTreeMap::new();
    new_map.insert("value".to_string(), JsonValue::Number(123.0));
    let new = JsonValue::Object(new_map);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}

#[test]
fn test_diff_null_values() {
    let mut old_map = BTreeMap::new();
    old_map.insert("a".to_string(), JsonValue::String("test".to_string()));
    old_map.insert("b".to_string(), JsonValue::Number(42.0));
    let old = JsonValue::Object(old_map);

    let mut new_map = BTreeMap::new();
    new_map.insert("a".to_string(), JsonValue::Null);
    new_map.insert("b".to_string(), JsonValue::Number(42.0));
    let new = JsonValue::Object(new_map);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}

#[test]
fn test_diff_boolean_values() {
    let mut old_map = BTreeMap::new();
    old_map.insert("enabled".to_string(), JsonValue::Bool(true));
    let old = JsonValue::Object(old_map);

    let mut new_map = BTreeMap::new();
    new_map.insert("enabled".to_string(), JsonValue::Bool(false));
    let new = JsonValue::Object(new_map);

    let patches = diff(&old, &new);

    let mut result = old.clone();

    for patch in patches {
        result.apply(patch).unwrap();
    }

    assert_eq!(result, new);
}