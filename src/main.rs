use jason::{parse_from_str, to_pretty_string};
use std::env;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [command, flag] if command == "format" && flag == "--stdin" => {
            std::process::exit(format_stdin());
        }
        _ => {
            eprintln!("Usage: jason format --stdin");
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
