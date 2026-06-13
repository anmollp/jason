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

#[test]
fn test_object_insertion() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let op = PatchOperation::Add {
        path: "/age".to_string(),
        value: JsonValue::Number(30.0),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(
        value.pointer("/age"),
        Some(&JsonValue::Number(30.0))
    );
}

#[test]
fn test_object_overwrite() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let op = PatchOperation::Add {
        path: "/name".to_string(),
        value: JsonValue::String("Jane".to_string()),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("Jane".to_string()))
    );
}

#[test]
fn test_array_middle_insertion() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(4.0),
    ]);

    let op = PatchOperation::Add {
        path: "/2".to_string(),
        value: JsonValue::Number(3.0),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(
        value,
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
            JsonValue::Number(4.0),
        ])
    );
}

#[test]
fn test_array_append() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    let op = PatchOperation::Add {
        path: "/-".to_string(),
        value: JsonValue::Number(4.0),
    };

    assert!(value.apply(op).is_ok());
    assert_eq!(
        value,
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
            JsonValue::Number(4.0),
        ])
    );
}

#[test]
fn test_array_out_of_bounds() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    let op = PatchOperation::Add {
        path: "/10".to_string(),
        value: JsonValue::Number(4.0),
    };

    assert!(value.apply(op).is_err());
}

#[test]
fn test_move_object_property() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    value.apply(PatchOperation::Move {
        from: "/name".to_string(),
        path: "/username".to_string(),
    }).unwrap();

    assert_eq!(value.pointer("/name"), None);

    assert_eq!(
        value.pointer("/username"),
        Some(&JsonValue::String("John".to_string()))
    );
}

#[test]
fn test_nested_move() {
    let mut preferences = BTreeMap::new();
    preferences.insert(
        "theme".to_string(),
        JsonValue::String("dark".to_string()),
    );

    let settings = BTreeMap::new();

    let mut root = BTreeMap::new();
    root.insert("preferences".to_string(), JsonValue::Object(preferences));
    root.insert("settings".to_string(), JsonValue::Object(settings));

    let mut value = JsonValue::Object(root);

    value.apply(PatchOperation::Move {
        from: "/preferences/theme".to_string(),
        path: "/settings/theme".to_string(),
    }).unwrap();

    assert_eq!(value.pointer("/preferences/theme"), None);
    assert_eq!(
        value.pointer("/settings/theme"),
        Some(&JsonValue::String("dark".to_string()))
    );
}

#[test]
fn test_array_move() {
    let mut value = JsonValue::Array(vec![
        JsonValue::String("a".to_string()),
        JsonValue::String("b".to_string()),
        JsonValue::String("c".to_string()),
    ]);

    value.apply(PatchOperation::Move {
        from: "/1".to_string(),
        path: "/0".to_string(),
    }).unwrap();

    assert_eq!(
        value,
        JsonValue::Array(vec![
            JsonValue::String("b".to_string()),
            JsonValue::String("a".to_string()),
            JsonValue::String("c".to_string()),
        ])
    );
}

#[test]
fn test_copy_object_property() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    value.apply(PatchOperation::Copy {
        from: "/name".to_string(),
        path: "/username".to_string(),
    }).unwrap();

    assert_eq!(
        value.pointer("/name"),
        Some(&JsonValue::String("John".to_string()))
    );
    assert_eq!(
        value.pointer("/username"),
        Some(&JsonValue::String("John".to_string()))
    );
}

#[test]
fn test_copy_array_element() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    value.apply(PatchOperation::Copy {
        from: "/1".to_string(),
        path: "/-".to_string(),
    }).unwrap();

    assert_eq!(
        value,
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
            JsonValue::Number(2.0),
        ])
    );
}

#[test]
fn test_copy_nested_value() {
    let mut inner = BTreeMap::new();
    inner.insert(
        "email".to_string(),
        JsonValue::String("john@example.com".to_string()),
    );

    let mut outer = BTreeMap::new();
    outer.insert("name".to_string(), JsonValue::String("John".to_string()));
    outer.insert("contact".to_string(), JsonValue::Object(inner));

    let mut value = JsonValue::Object(outer);

    value.apply(PatchOperation::Copy {
        from: "/contact/email".to_string(),
        path: "/email".to_string(),
    }).unwrap();

    assert_eq!(
        value.pointer("/email"),
        Some(&JsonValue::String("john@example.com".to_string()))
    );
    assert_eq!(
        value.pointer("/contact/email"),
        Some(&JsonValue::String("john@example.com".to_string()))
    );
}

#[test]
fn test_copy_invalid_source() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let result = value.apply(PatchOperation::Copy {
        from: "/missing".to_string(),
        path: "/username".to_string(),
    });

    assert!(result.is_err());
}

#[test]
fn test_test_operation_success() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let result = value.apply(PatchOperation::Test {
        path: "/name".to_string(),
        value: JsonValue::String("John".to_string()),
    });

    assert!(result.is_ok());
}

#[test]
fn test_test_operation_mismatch() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let result = value.apply(PatchOperation::Test {
        path: "/name".to_string(),
        value: JsonValue::String("Jane".to_string()),
    });

    assert!(result.is_err());
}

#[test]
fn test_test_operation_missing_path() {
    let mut obj = BTreeMap::new();
    obj.insert(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    );

    let mut value = JsonValue::Object(obj);

    let result = value.apply(PatchOperation::Test {
        path: "/age".to_string(),
        value: JsonValue::Number(30.0),
    });

    assert!(result.is_err());
}

#[test]
fn test_test_operation_nested_value() {
    let mut inner = BTreeMap::new();
    inner.insert(
        "theme".to_string(),
        JsonValue::String("dark".to_string()),
    );

    let mut outer = BTreeMap::new();
    outer.insert("preferences".to_string(), JsonValue::Object(inner));

    let mut value = JsonValue::Object(outer);

    let result = value.apply(PatchOperation::Test {
        path: "/preferences/theme".to_string(),
        value: JsonValue::String("dark".to_string()),
    });

    assert!(result.is_ok());
}

#[test]
fn test_test_operation_array_element() {
    let mut value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);

    let result = value.apply(PatchOperation::Test {
        path: "/1".to_string(),
        value: JsonValue::Number(2.0),
    });

    assert!(result.is_ok());
}