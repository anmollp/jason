use crate::JsonValue;

pub fn to_json_string(value: &JsonValue) -> String {
    let mut output = String::new();
    serialize(value, &mut output);
    output
}

fn serialize(value: &JsonValue, output: &mut String) {
    match value {
        JsonValue::Null => output.push_str("null"),
        JsonValue::Bool(b) => match b {
            true => output.push_str("true"),
            false => output.push_str("false"),
        },
        JsonValue::Number(n) => output.push_str(format!("{}", n).as_str()),
        JsonValue::String(s) => write_string(s, output),
        JsonValue::Array(values) => {
            output.push('[');
            let mut iter = values.iter();
            if let Some(first) = iter.next() {
                serialize(first, output);
                for v in iter {
                    output.push(',');
                    serialize(v, output);
                }
            }
            output.push(']');
        }
        JsonValue::Object(map) => {
            output.push('{');
            let mut iter = map.iter();
            if let Some((key, value)) = iter.next() {
                write_string(key, output);
                output.push(':');
                serialize(value, output);
                for (key, value) in iter {
                    output.push(',');
                    write_string(key, output);
                    output.push(':');
                    serialize(value, output);
                }
            }
            output.push('}');
        }
    }
}

pub fn to_pretty_string(value: &JsonValue) -> String {
    let mut output = String::new();
    pretty_serialize(value, &mut output, 0);
    output
}

fn pretty_serialize(value: &JsonValue, output: &mut String, indent: usize) {
    match value {
        JsonValue::Null => output.push_str("null"),
        JsonValue::Number(n) => output.push_str(&n.to_string()),
        JsonValue::String(s) => write_string(s, output),
        JsonValue::Bool(b) => match b {
            true => output.push_str("true"),
            false => output.push_str("false"),
        },
        JsonValue::Array(a) => {
            output.push('[');
            if a.is_empty() {
                output.push(']');
                return;
            }
            let len = a.len();
            for (i, item) in a.iter().enumerate() {
                output.push('\n');
                write_indent(output, indent + 1);
                pretty_serialize(item, output, indent + 1);
                if i + 1 != len {
                    output.push(',');
                }
            }
            output.push('\n');
            write_indent(output, indent);
            output.push(']')
        }
        JsonValue::Object(o) => {
            output.push('{');
            if o.is_empty() {
                output.push('}');
                return;
            }
            let len = o.len();
            for (i, (key, value)) in o.iter().enumerate() {
                output.push('\n');
                write_indent(output, indent + 1);
                write_string(key, output);
                output.push_str(": ");
                pretty_serialize(value, output, indent + 1);
                if i + 1 != len {
                    output.push(',');
                }
            }
            output.push('\n');
            write_indent(output, indent);
            output.push('}')
        }
    }
}

fn write_indent(output: &mut String, indent: usize) {
    for _ in 0..indent {
        output.push_str("  ")
    }
}

fn write_string(s: &str, output: &mut String) {
    output.push('"');
    for ch in s.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\t' => output.push_str("\\t"),
            _ => output.push(ch),
        }
    }
    output.push('"');
}
