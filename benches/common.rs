#![allow(dead_code)]
use std::collections::BTreeMap;
use jason::{JsonValue, PatchOperation};


pub fn medium_json() -> String {
    let mut s = String::from("{\"users\":[");
    for i in 0..100 {
        s.push_str(&format!(
            "{{\"id\":{},\"name\":\"user{}\",\"active\":true}},",
            i, i
        ));
    }
    s.pop();
    s.push_str("]}");
    s
}

pub fn large_json() -> String {
    let mut s = String::from("{\"users\":[");
    for i in 0..10_000 {
        s.push_str(&format!(
            "{{\"id\":{},\"name\":\"user{}\",\"active\":true}},",
            i, i
        ));
    }
    s.pop();
    s.push_str("]}");
    s
}

pub fn large_document() -> JsonValue {
    let users = (0..50_000)
        .map(|i| {
            let mut user = BTreeMap::new();
            user.insert("name".to_string(), JsonValue::String(format!("user{}", i)));
            user.insert("active".to_string(), JsonValue::Bool(true));
            JsonValue::Object(user)
        })
        .collect();

    let mut root = BTreeMap::new();
    root.insert("users".to_string(), JsonValue::Array(users));
    JsonValue::Object(root)
}

pub fn make_array_json(size: usize, changed_index: usize) -> (JsonValue, JsonValue) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 0..size {
        a.push(JsonValue::Number(i as f64));

        if i == changed_index {
            b.push(JsonValue::Number(999999.0));
        } else {
            b.push(JsonValue::Number(i as f64));
        }
    }

    (
        JsonValue::Array(a),
        JsonValue::Array(b),
    )
}

pub fn make_object_json(size: usize, changed_index: usize) -> (JsonValue, JsonValue) {
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();

    for i in 0..size {
        a.insert(format!("key{}", i), JsonValue::Number(i as f64));

        if i == changed_index {
            b.insert(format!("key{}", i), JsonValue::Number(9999.0));
        } else {
            b.insert(format!("key{}", i), JsonValue::Number(i as f64));
        }
    }

    (
        JsonValue::Object(a),
        JsonValue::Object(b),
    )
}

pub fn make_replace_operations(size: usize) -> Vec<PatchOperation> {
    (0..size)
        .map(|i| PatchOperation::Replace {
            path: format!("/users/{}/name", i),
            value: JsonValue::String(format!("changed{}", i)),
        })
        .collect()
}

pub fn make_remove_operations(size: usize) -> Vec<PatchOperation> {
    (0..size)
        .map(|i| PatchOperation::Remove {
            path: format!("/users/{}/active", i),
        })
        .collect()
}