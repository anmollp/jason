use crate::JsonValue;

pub enum PatchOperation {
    Replace {
        path: String,
        value: JsonValue
    },
    Remove {
        path: String,
    }
}

pub enum PatchError {
    InvalidPath,
    MissingValue,
    InvalidArrayIndex,
    IndexOutOfBounds
}