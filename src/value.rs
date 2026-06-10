use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn pointer(&self, pointer: &str) -> Option<&JsonValue> {
        if pointer.is_empty() {
            return Some(self);
        }

        if !pointer.starts_with('/') {
            return None;
        }

        let mut current = self;
        for segment in pointer.split('/').skip(1) {
            match current {
                JsonValue::Object(map) => {
                    let key = decode_pointer_segment(segment);
                    current = map.get(key.as_str())?;
                }
                JsonValue::Array(array) => {
                    let index: usize = segment.parse().ok()?;
                    current = array.get(index)?;
                }
                _ => return None,
            }
        }
        Some(current)
    }

    pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut JsonValue> {
        if pointer.is_empty() {
            return Some(self);
        }

        if !pointer.starts_with('/') {
            return None;
        }

        let mut current = self;
        for segment in pointer.split('/').skip(1) {
            match current {
                JsonValue::Object(map) => {
                    let key = decode_pointer_segment(segment);
                    current = map.get_mut(key.as_str())?;
                }
                JsonValue::Array(array) => {
                    let index: usize = segment.parse().ok()?;
                    current = array.get_mut(index)?;
                }
                _ => return None,
            }
        }
        Some(current)
    }
}

fn decode_pointer_segment(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}
