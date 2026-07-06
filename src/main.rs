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
        _ => {
            eprintln!("Usage: jason format --stdin\n       jason diff --stdin");
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
