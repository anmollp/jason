use jason::{JsonValue, PatchOperation, diff, parse_from_str, to_json_string, to_pretty_string};
use std::env;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [command, flag] if command == "format" && flag == "--stdin" => {
            std::process::exit(format_stdin());
        }
        [command, flag] if command == "diff" && flag == "--stdin" => {
            std::process::exit(diff_stdin());
        }
        [command, flag] if command == "patch" && flag == "--stdin" => {
            std::process::exit(patch_stdin());
        }
        [command, flag] if command == "pointer" && flag == "--stdin" => {
            std::process::exit(pointer_stdin());
        }
        _ => {
            eprintln!(
                "Usage: jason format --stdin\n       jason diff --stdin\n       jason patch --stdin\n       jason pointer --stdin"
            );
            std::process::exit(2);
        }
    }
}

fn format_stdin() -> i32 {
    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {err}");
        return 1;
    }

    let value = match parse_from_str(&input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{err}");
            return 1;
        }
    };

    if let Err(err) = writeln!(io::stdout(), "{}", to_pretty_string(&value)) {
        eprintln!("failed to write stdout: {err}");
        return 1;
    }

    0
}

fn diff_stdin() -> i32 {
    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {err}");
        return 1;
    }

    let Some((old_input, new_input)) = input.split_once('\0') else {
        eprintln!("diff stdin payload must contain one NUL separator");
        return 1;
    };

    let old_value = match parse_from_str(old_input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("before: {err}");
            return 1;
        }
    };
    let new_value = match parse_from_str(new_input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("after: {err}");
            return 1;
        }
    };

    let patches = diff(&old_value, &new_value);
    let output = patch_operations_to_json(&patches);

    if let Err(err) = writeln!(io::stdout(), "{output}") {
        eprintln!("failed to write stdout: {err}");
        return 1;
    }

    0
}

fn patch_stdin() -> i32 {
    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {err}");
        return 1;
    }

    let Some((document_input, patch_input)) = input.split_once('\0') else {
        eprintln!("patch stdin payload must contain one NUL separator");
        return 1;
    };

    let mut document = match parse_from_str(document_input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("document: {err}");
            return 1;
        }
    };

    let patch_value = match parse_from_str(patch_input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("patch: {err}");
            return 1;
        }
    };

    let operations = match json_to_patch_operations(patch_value) {
        Ok(operations) => operations,
        Err(err) => {
            eprintln!("patch: {err}");
            return 1;
        }
    };

    for operation in operations {
        if let Err(err) = document.apply(operation) {
            eprintln!("patch: {err:?}");
            return 1;
        }
    }

    if let Err(err) = writeln!(io::stdout(), "{}", to_pretty_string(&document)) {
        eprintln!("failed to write stdout: {err}");
        return 1;
    }

    0
}

fn pointer_stdin() -> i32 {
    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {err}");
        return 1;
    }

    let Some((document_input, pointer_path)) = input.split_once('\0') else {
        eprintln!("pointer stdin payload must contain one NUL separator");
        return 1;
    };

    let document = match parse_from_str(document_input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("document: {err}");
            return 1;
        }
    };

    let Some(value) = document.pointer(pointer_path.trim()) else {
        eprintln!("pointer: path not found");
        return 1;
    };

    if let Err(err) = writeln!(io::stdout(), "{}", to_pretty_string(value)) {
        eprintln!("failed to write stdout: {err}");
        return 1;
    }

    0
}

fn patch_operations_to_json(patches: &[PatchOperation]) -> String {
    if patches.is_empty() {
        return "[]".to_string();
    }

    let lines = patches
        .iter()
        .map(|patch| format!("  {}", patch_operation_to_json(patch)))
        .collect::<Vec<_>>()
        .join(",\n");

    format!("[\n{lines}\n]")
}

fn patch_operation_to_json(patch: &PatchOperation) -> String {
    match patch {
        PatchOperation::Add { path, value } => format!(
            "{{\"op\":\"add\",\"path\":{},\"value\":{}}}",
            json_string(path),
            to_json_string(value)
        ),
        PatchOperation::Remove { path } => {
            format!("{{\"op\":\"remove\",\"path\":{}}}", json_string(path))
        }
        PatchOperation::Replace { path, value } => format!(
            "{{\"op\":\"replace\",\"path\":{},\"value\":{}}}",
            json_string(path),
            to_json_string(value)
        ),
        PatchOperation::Move { from, path } => format!(
            "{{\"op\":\"move\",\"from\":{},\"path\":{}}}",
            json_string(from),
            json_string(path)
        ),
        PatchOperation::Copy { from, path } => format!(
            "{{\"op\":\"copy\",\"from\":{},\"path\":{}}}",
            json_string(from),
            json_string(path)
        ),
        PatchOperation::Test { path, value } => format!(
            "{{\"op\":\"test\",\"path\":{},\"value\":{}}}",
            json_string(path),
            to_json_string(value)
        ),
    }
}

fn json_string(value: &str) -> String {
    to_json_string(&JsonValue::String(value.to_string()))
}

fn json_to_patch_operations(value: JsonValue) -> Result<Vec<PatchOperation>, String> {
    let JsonValue::Array(items) = value else {
        return Err("expected an array of JSON Patch operations".to_string());
    };

    items
        .into_iter()
        .enumerate()
        .map(|(index, item)| json_to_patch_operation(item, index))
        .collect()
}

fn json_to_patch_operation(value: JsonValue, index: usize) -> Result<PatchOperation, String> {
    let JsonValue::Object(mut object) = value else {
        return Err(format!("operation {index} must be an object"));
    };

    let op = take_string(&mut object, "op", index)?;
    let path = take_string(&mut object, "path", index)?;

    match op.as_str() {
        "add" => Ok(PatchOperation::Add {
            path,
            value: take_value(&mut object, "value", index)?,
        }),
        "remove" => Ok(PatchOperation::Remove { path }),
        "replace" => Ok(PatchOperation::Replace {
            path,
            value: take_value(&mut object, "value", index)?,
        }),
        "move" => Ok(PatchOperation::Move {
            from: take_string(&mut object, "from", index)?,
            path,
        }),
        "copy" => Ok(PatchOperation::Copy {
            from: take_string(&mut object, "from", index)?,
            path,
        }),
        "test" => Ok(PatchOperation::Test {
            path,
            value: take_value(&mut object, "value", index)?,
        }),
        _ => Err(format!("operation {index} has unsupported op {op:?}")),
    }
}

fn take_string(
    object: &mut std::collections::BTreeMap<String, JsonValue>,
    key: &str,
    index: usize,
) -> Result<String, String> {
    match object.remove(key) {
        Some(JsonValue::String(value)) => Ok(value),
        Some(_) => Err(format!("operation {index} field {key:?} must be a string")),
        None => Err(format!("operation {index} is missing field {key:?}")),
    }
}

fn take_value(
    object: &mut std::collections::BTreeMap<String, JsonValue>,
    key: &str,
    index: usize,
) -> Result<JsonValue, String> {
    object
        .remove(key)
        .ok_or_else(|| format!("operation {index} is missing field {key:?}"))
}
