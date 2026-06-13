use crate::JsonValue;

pub enum PatchOperation {
    Replace {
        path: String,
        value: JsonValue
    },
    Remove {
        path: String,
    },
    Add {
        path: String,
        value: JsonValue
    },
    Move {
        from: String,
        path: String
    },
    Copy {
        from: String,
        path: String
    },
    Test {
        path: String,
        value: JsonValue
    }
}

#[derive(Debug)]
pub enum PatchError {
    InvalidPath,
    MissingValue,
    InvalidArrayIndex,
    IndexOutOfBounds,
    TestFailed
}