use std::io::Write;
use std::process::{Command, Stdio};

fn jason_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_jason"))
}

#[test]
fn format_stdin_pretty_prints_json() {
    let mut child = jason_command()
        .args(["format", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jason CLI");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(br#"{"a":1}"#)
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait for jason CLI");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout is UTF-8"),
        "{\n  \"a\": 1\n}\n"
    );
    assert_eq!(output.stderr, b"");
}

#[test]
fn format_stdin_reports_parse_errors_on_stderr() {
    let mut child = jason_command()
        .args(["format", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jason CLI");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(b"{true: 1}")
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait for jason CLI");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("Expected a string key"));
}

#[test]
fn diff_stdin_prints_patch_operations() {
    let mut child = jason_command()
        .args(["diff", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jason CLI");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(br#"{"a":1}"#)
        .expect("write before JSON");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(b"\0")
        .expect("write separator");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(br#"{"a":2,"b":true}"#)
        .expect("write after JSON");

    let output = child.wait_with_output().expect("wait for jason CLI");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout is UTF-8"),
        "[\n  {\"op\":\"replace\",\"path\":\"/a\",\"value\":2},\n  {\"op\":\"add\",\"path\":\"/b\",\"value\":true}\n]\n"
    );
    assert_eq!(output.stderr, b"");
}

#[test]
fn diff_stdin_reports_which_side_failed_to_parse() {
    let mut child = jason_command()
        .args(["diff", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jason CLI");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(br#"{"a":1}"#)
        .expect("write before JSON");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(b"\0")
        .expect("write separator");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(b"{true: 1}")
        .expect("write after JSON");

    let output = child.wait_with_output().expect("wait for jason CLI");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("after:"));
    assert!(stderr.contains("Expected a string key"));
}

#[test]
fn invalid_usage_exits_with_usage_error() {
    let output = jason_command()
        .args(["format"])
        .output()
        .expect("run jason CLI");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert_eq!(
        stderr,
        "Usage: jason format --stdin\n       jason diff --stdin\n"
    );
}
