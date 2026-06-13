use std::cmp::PartialEq;
use std::collections::BTreeMap;
use crate::patch::PatchOperation;
use crate::PatchError;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn apply(&mut self, operation: PatchOperation) -> Result<(), PatchError> {
        match operation {
            PatchOperation::Replace { path, value} => {
                self.replace(&path, value)
                    .then_some(())
                    .ok_or(PatchError::InvalidPath)
            },
            PatchOperation::Remove { path } => {
                self.remove(&path)?;
                    Ok(())
            },
            PatchOperation::Add { path, value} => {
                self.add(&path, value)?;
                Ok(())
            },
            PatchOperation::Move { from, path} => {
                self.r#move(&from, &path)?;
                Ok(())
            },
            PatchOperation::Copy { from, path} => {
                self.copy(&from, &path)?;
                Ok(())
            },
            PatchOperation::Test { path, value } => {
                self.test(&path, value)?;
                Ok(())
            }
        }
    }

    pub fn replace(&mut self, path: &str, value: JsonValue) -> bool {
        match self.pointer_mut(path) {
            Some(target) => {
                *target = value;
                true
            }
            None => false,
        }
    }

    pub fn remove(&mut self, path: &str) -> Result<JsonValue, PatchError> {
        let (parent_path, child) = match split_parent(path) {
          Some(parts) => parts,
            None => return Err(PatchError::InvalidPath)
        };
        let parent = match self.pointer_mut(parent_path) {
            Some(value) => value,
            None => return Err(PatchError::InvalidPath)
        };
        match parent {
            JsonValue::Array(arr) => {
                let index = child.parse::<usize>().map_err(|_| PatchError::InvalidArrayIndex)?;
                if index >= arr.len() {
                    return Err(PatchError::IndexOutOfBounds);
                }
                Ok(arr.remove(index))
            },
            JsonValue::Object(map) => {
                match map.remove(child) {
                    Some(value) => Ok(value),
                    None => Err(PatchError::MissingValue)
                }
            },
            _ => Err(PatchError::InvalidPath)
        }
    }

    pub fn add(&mut self, path: &str, value: JsonValue) -> Result<(), PatchError> {
        let (parent_path, child) = match split_parent(path) {
            Some(parts) => parts,
            None => return Err(PatchError::InvalidPath)
        };
        let parent = match self.pointer_mut(parent_path) {
            Some(val) => val,
            None => return Err(PatchError::InvalidPath)
        };
        match parent {
            JsonValue::Object(obj) => {
                obj.insert(child.to_string(), value);
                Ok(())
            },
            JsonValue::Array(arr) => {
                if child == "-" {
                    arr.push(value);
                    Ok(())
                } else {
                    let index = child.parse::<usize>().map_err(|_| PatchError::InvalidArrayIndex)?;
                    if index > arr.len() {
                        return Err(PatchError::IndexOutOfBounds);
                    }
                    arr.insert(index, value);
                    Ok(())
                }
            },
            _ => Err(PatchError::InvalidPath)
        }
    }

    pub fn r#move(&mut self, from: &str, path: &str) -> Result<(), PatchError> {
        let value = self.remove(from)?;
        self.add(path, value)
    }

    pub fn copy(&mut self, from: &str, path: &str) -> Result<(), PatchError> {
        let value = self
            .pointer(from)
            .ok_or(PatchError::InvalidPath)?
            .clone();

        self.add(path, value)
    }

    pub fn test(&mut self, path: &str, expected_value: JsonValue) -> Result<(), PatchError> {
        let actual = match self.pointer(path) {
            Some(value) => value,
            None => return Err(PatchError::InvalidPath)
        };
        if actual == &expected_value {
            Ok(())
        } else {
            Err(PatchError::TestFailed)
        }
    }
}

fn decode_pointer_segment(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

fn split_parent(pointer: &str) -> Option<(&str, &str)> {
    let mut parts = pointer.rsplitn(2, "/");
    let child = parts.next()?;
    let parent = parts.next()?;
    Some((parent, child))
}