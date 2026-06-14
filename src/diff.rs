use crate::{JsonValue, PatchOperation};

pub fn diff(old: &JsonValue, new: &JsonValue) -> Vec<PatchOperation> {
    let mut patches = Vec::new();
    diff_inner(old, new, "", &mut patches);
    patches
}

fn diff_inner(old: &JsonValue, new: &JsonValue, path: &str, patches: &mut Vec<PatchOperation>) {
    match (old, new) {
        (JsonValue::Object(old_map), JsonValue::Object(new_map)) => {
            for key in old_map.keys() {
                if !new_map.contains_key(key) {
                    patches.push(PatchOperation::Remove {
                        path: join_path(path, key),
                    })
                }
                if let Some(new_value) = new_map.get(key) {
                    let old_value = old_map.get(key).unwrap();
                    diff_inner(old_value, new_value, &join_path(path, key), patches)
                };
            }
            for key in new_map.keys() {
                if !old_map.contains_key(key) {
                    patches.push(PatchOperation::Add {
                        path: join_path(path, key),
                        value: new_map.get(key).unwrap().to_owned(),
                    })
                }
            }
        }
        (JsonValue::Array(_), JsonValue::Array(_)) => {
            if old != new {
                patches.push(PatchOperation::Replace {
                    path: path.to_string(),
                    value: new.clone(),
                });
            }
        }
        _ => {
            if old != new {
                patches.push(PatchOperation::Replace {
                    path: path.to_string(),
                    value: new.clone(),
                });
            }
        }
    }
}

fn join_path(base: &str, segment: &str) -> String {
    let path = vec![base, segment];
    path.join("/")
}
