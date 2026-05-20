use crate::JsonError;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null
}

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize
}

#[derive(Debug, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter {
        ch: char,
        position: Position
    },
    UnterminatedString (Position),
    InvalidNumber (Position),
    UnexpectedLiteral (Position),
    UnterminatedNumber (Position),
    InvalidUnicodeEscape (Position),
    UnexpectedEndOfInput (Position)
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedEndOfInput(Position { line, column }) =>
                write!(f, "Unexpected end of input at line {line}, column {column}"),
            LexerError::UnexpectedCharacter{
                ch,
                position: Position {line, column}
            } => write!(f, "Unexpected character '{ch}' at line {line} column {column}"),
            LexerError::InvalidNumber(Position { line, column }) =>
                write!(f, "Invalid number at line {line}, column {column}"),
            LexerError::InvalidUnicodeEscape(Position { line, column}) =>
                write!(f, "Invalid unicode escape at line {line}, column {column}"),
            LexerError::UnexpectedLiteral(Position{ line, column}) =>
                write!(f, "Unexpected literal at line {line}, column {column}"),
            LexerError::UnterminatedNumber(Position{ line, column}) =>
                write!(f, "Unterminated number at line {line}, column {column}"),
            LexerError::UnterminatedString(Position { line, column}) =>
                write!(f, "Unterminated string at line {line}, column {column}")
        }
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer{
            chars: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.position += 1
            }
            else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, JsonError> {
        self.skip_whitespace();
        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        match ch {
            '{' => {self.next(); Ok(Some(Token::LeftBrace))},
            '}' => {self.next(); Ok(Some(Token::RightBrace))},
            '[' => {self.next(); Ok(Some(Token::LeftBracket))},
            ']' => {self.next(); Ok(Some(Token::RightBracket))},
            ':' => {self.next(); Ok(Some(Token::Colon))},
            ',' => {self.next(); Ok(Some(Token::Comma))},
            '"' => Ok(Some(Token::String(self.read_string()?))),
            't' => {self.read_literal("true")?; Ok(Some(Token::True))},
            'f' => {self.read_literal("false")?; Ok(Some(Token::False))},
            'n' => {self.read_literal("null")?; Ok(Some(Token::Null))},
            c if c.is_ascii_digit() || c == '-' => Ok(Some(Token::Number(self.read_number()?))),
            _ => Err(JsonError::Lexer(LexerError::UnexpectedCharacter{
                ch,
                position: Position{
                line: self.line,
                column: self.column
            }
        })),
        }
    }

    fn read_string(&mut self) -> Result<String, JsonError> {
        let mut string_token = String::new();
        self.next();
        while let Some(ch) = self.next() {
            match ch {
                '"' => return Ok(string_token),
                '\\' => {
                    let escaped = match self.next() {
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('\\') => '\\',
                        Some('"') => '"',
                        Some('u') => self.read_unicode_escape()?,
                        Some(_other) => return Err(JsonError::Lexer(LexerError::InvalidUnicodeEscape(Position{
                            line: self.line,
                            column: self.column
                        }))),
                        None => return Err(JsonError::Lexer(LexerError::UnexpectedEndOfInput(Position{
                            line: self.line,
                            column: self.column
                        })))
                    };
                    string_token.push(escaped);
                }
                _ => string_token.push(ch)
            };
        }
        Err(JsonError::Lexer(LexerError::UnterminatedString(Position{
            line: self.line,
            column: self.column
        })))
    }

    fn read_unicode_escape(&mut self) -> Result<char, JsonError> {
        let mut hex = String::new();
        for _ in 0..4 {
            match self.next() {
                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                _ => return Err(JsonError::Lexer(LexerError::InvalidUnicodeEscape(Position{
                    line: self.line,
                    column: self.column
                })))
            }
        }
        let code_point = u32::from_str_radix(&hex, 16)
                .map_err(|_| JsonError::Lexer(LexerError::InvalidUnicodeEscape(Position{
                    line: self.line,
                    column: self.column
                })))?;
        char::from_u32(code_point)
            .ok_or(JsonError::Lexer(LexerError::InvalidUnicodeEscape(Position{
                line: self.line,
                column: self.column
            })))
    }

    fn read_number(&mut self) -> Result<f64, JsonError> {
        let mut number = String::new();
        if self.peek() == Some('-') {
            number.push('-');
            self.next();
        }

        let mut seen_dot = false;
        let mut digit_after_dot = false;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                if seen_dot {
                    digit_after_dot = true;
                }
                number.push(ch);
                self.next();
            }
            else if ch == '.' && !seen_dot  {
                number.push(ch);
                seen_dot = true;
                self.next();
            }
            else {
                break;
            }
        };
        if seen_dot && !digit_after_dot {
            return Err(JsonError::Lexer(LexerError::UnterminatedNumber(Position{
                line: self.line,
                column: self.column
            })))
        }
        number
            .parse::<f64>()
            .map_err(|_| JsonError::Lexer(LexerError::InvalidNumber(Position{
                line: self.line,
                column: self.column
            })))
    }

    fn read_literal(&mut self, expected: &str) -> Result<(), JsonError> {
        for expected_char in expected.chars() {
            match self.next() {
                Some(c) if c == expected_char => {},
                _ => return Err(JsonError::Lexer(LexerError::UnexpectedLiteral(Position{
                    line: self.line,
                    column: self.column
                }))),
            }
        }
        Ok(())
    }
}