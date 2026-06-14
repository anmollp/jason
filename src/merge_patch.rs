use crate::JsonValue;

pub fn merge_patch(target: &mut JsonValue, patch: &JsonValue) {
    match patch {
        // Case 1: patch is an object -> deep merge
        JsonValue::Object(patch_map) => {
            if let JsonValue::Object(target_map) = target {
                for (key, patch_value) in patch_map {
                    match patch_value {
                        // null = delete
                        JsonValue::Null => {
                            target_map.remove(key);
                        },
                        // object = recurse if possible
                        _ => match target_map.get_mut(key) {
                                Some(child) => merge_patch(child, patch_value),
                                None => {
                                    target_map.insert(key.clone(), patch_value.clone());
                                }
                            }
                    }
                }
            } else {
                // target is not object → replace
                *target = patch.clone()
            }
        },
        // patch is not object → replace entire target
        _ => *target = patch.clone()
    }
}