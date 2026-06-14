use std::collections::BTreeMap;
use jason::{JsonValue, merge_patch};

#[test]
fn test_merge_patch_simple_object() {
    let mut target_map = BTreeMap::new();
    target_map.insert("name".to_string(), JsonValue::String("John".to_string()));
    target_map.insert("age".to_string(), JsonValue::Number(30.0));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("age".to_string(), JsonValue::Number(31.0));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/name"), Some(&JsonValue::String("John".to_string())));
    assert_eq!(target.pointer("/age"), Some(&JsonValue::Number(31.0)));
}

#[test]
fn test_merge_patch_add_new_field() {
    let mut target_map = BTreeMap::new();
    target_map.insert("name".to_string(), JsonValue::String("John".to_string()));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("email".to_string(), JsonValue::String("john@example.com".to_string()));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/name"), Some(&JsonValue::String("John".to_string())));
    assert_eq!(target.pointer("/email"), Some(&JsonValue::String("john@example.com".to_string())));
}

#[test]
fn test_merge_patch_delete_field_with_null() {
    let mut target_map = BTreeMap::new();
    target_map.insert("name".to_string(), JsonValue::String("John".to_string()));
    target_map.insert("age".to_string(), JsonValue::Number(30.0));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("age".to_string(), JsonValue::Null);

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/name"), Some(&JsonValue::String("John".to_string())));
    assert_eq!(target.pointer("/age"), None);
}

#[test]
fn test_merge_patch_nested_object() {
    let mut target_inner = BTreeMap::new();
    target_inner.insert("street".to_string(), JsonValue::String("123 Main St".to_string()));
    target_inner.insert("city".to_string(), JsonValue::String("New York".to_string()));

    let mut target_map = BTreeMap::new();
    target_map.insert("name".to_string(), JsonValue::String("John".to_string()));
    target_map.insert("address".to_string(), JsonValue::Object(target_inner));

    let mut target = JsonValue::Object(target_map);

    let mut patch_inner = BTreeMap::new();
    patch_inner.insert("city".to_string(), JsonValue::String("Los Angeles".to_string()));

    let mut patch_map = BTreeMap::new();
    patch_map.insert("address".to_string(), JsonValue::Object(patch_inner));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/name"), Some(&JsonValue::String("John".to_string())));
    assert_eq!(target.pointer("/address/street"), Some(&JsonValue::String("123 Main St".to_string())));
    assert_eq!(target.pointer("/address/city"), Some(&JsonValue::String("Los Angeles".to_string())));
}

#[test]
fn test_merge_patch_deeply_nested() {
    let mut target_level3 = BTreeMap::new();
    target_level3.insert("country".to_string(), JsonValue::String("USA".to_string()));

    let mut target_level2 = BTreeMap::new();
    target_level2.insert("city".to_string(), JsonValue::String("New York".to_string()));
    target_level2.insert("info".to_string(), JsonValue::Object(target_level3));

    let mut target_map = BTreeMap::new();
    target_map.insert("address".to_string(), JsonValue::Object(target_level2));

    let mut target = JsonValue::Object(target_map);

    let mut patch_level3 = BTreeMap::new();
    patch_level3.insert("country".to_string(), JsonValue::String("Canada".to_string()));

    let mut patch_level2 = BTreeMap::new();
    patch_level2.insert("info".to_string(), JsonValue::Object(patch_level3));

    let mut patch_map = BTreeMap::new();
    patch_map.insert("address".to_string(), JsonValue::Object(patch_level2));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/address/city"), Some(&JsonValue::String("New York".to_string())));
    assert_eq!(target.pointer("/address/info/country"), Some(&JsonValue::String("Canada".to_string())));
}

#[test]
fn test_merge_patch_replace_object_with_value() {
    let mut target_map = BTreeMap::new();
    let inner = BTreeMap::new();
    target_map.insert("data".to_string(), JsonValue::Object(inner));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("data".to_string(), JsonValue::String("simple string".to_string()));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/data"), Some(&JsonValue::String("simple string".to_string())));
}

#[test]
fn test_merge_patch_replace_non_object_target() {
    let mut target = JsonValue::String("old value".to_string());

    let mut patch_map = BTreeMap::new();
    patch_map.insert("name".to_string(), JsonValue::String("new".to_string()));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/name"), Some(&JsonValue::String("new".to_string())));
}

#[test]
fn test_merge_patch_replace_with_scalar() {
    let mut target_map = BTreeMap::new();
    target_map.insert("field".to_string(), JsonValue::String("value".to_string()));

    let mut target = JsonValue::Object(target_map);

    let patch = JsonValue::Number(42.0);

    merge_patch(&mut target, &patch);

    assert_eq!(target, JsonValue::Number(42.0));
}

#[test]
fn test_merge_patch_empty_patch() {
    let mut target_map = BTreeMap::new();
    target_map.insert("name".to_string(), JsonValue::String("John".to_string()));

    let target_clone = JsonValue::Object(target_map.clone());
    let mut target = JsonValue::Object(target_map);

    let patch_map = BTreeMap::new();
    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target, target_clone);
}

#[test]
fn test_merge_patch_multiple_changes() {
    let mut target_map = BTreeMap::new();
    target_map.insert("a".to_string(), JsonValue::Number(1.0));
    target_map.insert("b".to_string(), JsonValue::Number(2.0));
    target_map.insert("c".to_string(), JsonValue::Number(3.0));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("a".to_string(), JsonValue::Number(10.0));
    patch_map.insert("b".to_string(), JsonValue::Null);
    patch_map.insert("d".to_string(), JsonValue::Number(4.0));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/a"), Some(&JsonValue::Number(10.0)));
    assert_eq!(target.pointer("/b"), None);
    assert_eq!(target.pointer("/c"), Some(&JsonValue::Number(3.0)));
    assert_eq!(target.pointer("/d"), Some(&JsonValue::Number(4.0)));
}

#[test]
fn test_merge_patch_boolean_values() {
    let mut target_map = BTreeMap::new();
    target_map.insert("enabled".to_string(), JsonValue::Bool(false));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("enabled".to_string(), JsonValue::Bool(true));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/enabled"), Some(&JsonValue::Bool(true)));
}

#[test]
fn test_merge_patch_null_value_assignment() {
    let mut target_map = BTreeMap::new();
    target_map.insert("field".to_string(), JsonValue::String("value".to_string()));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("field".to_string(), JsonValue::Null);

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(target.pointer("/field"), None);
}

#[test]
fn test_merge_patch_array_replacement() {
    let mut target_map = BTreeMap::new();
    target_map.insert("items".to_string(), JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
    ]));

    let mut target = JsonValue::Object(target_map);

    let mut patch_map = BTreeMap::new();
    patch_map.insert("items".to_string(), JsonValue::Array(vec![
        JsonValue::Number(3.0),
        JsonValue::Number(4.0),
        JsonValue::Number(5.0),
    ]));

    let patch = JsonValue::Object(patch_map);

    merge_patch(&mut target, &patch);

    assert_eq!(
        target.pointer("/items"),
        Some(&JsonValue::Array(vec![
            JsonValue::Number(3.0),
            JsonValue::Number(4.0),
            JsonValue::Number(5.0),
        ]))
    );
}
