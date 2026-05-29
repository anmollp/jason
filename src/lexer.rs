use crate::{JsonError, Position};
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

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub position: Position
}

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize
}

#[derive(Debug)]
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
    UnexpectedEndOfInput (Position),
    InvalidEscapeCharacter {
        ch: char,
        position: Position
    },
    LeadingZero(Position)
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
                write!(f, "Unterminated string at line {line}, column {column}"),
            LexerError::InvalidEscapeCharacter {
                ch,
                position: Position{ line, column}
            } => write!(f, "Invalid escape character '{ch}' at line {line}, column {column}"),
            LexerError::LeadingZero(Position { line, column}) =>
                write!(f, "Leading zero at line {line}, column {column}"),
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

    fn current_positon(&self) -> Position {
        Position {
            line: self.line,
            column: self.column
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

    fn make_token(&self, token:Token, position: Position) -> SpannedToken {
        SpannedToken {
            token,
            position
        }
    }

    pub fn next_token(&mut self) -> Result<Option<SpannedToken>, JsonError> {
        self.skip_whitespace();
        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        let position = self.current_positon();

        match ch {
            '{' => {self.next(); Ok(Some(self.make_token(Token::LeftBrace, position))) },
            '}' => {self.next(); Ok(Some(self.make_token(Token::RightBrace, position))) },
            '[' => {self.next(); Ok(Some(self.make_token(Token::LeftBracket, position))) },
            ']' => {self.next(); Ok(Some(self.make_token(Token::RightBracket, position)))},
            ':' => {self.next(); Ok(Some(self.make_token(Token::Colon, position)))},
            ',' => {self.next(); Ok(Some(self.make_token(Token::Comma, position)))},
            '"' => { let string = self.read_string()?;
                Ok(Some(self.make_token(Token::String(string), position)))
            },
            't' => {self.read_literal("true")?; Ok(Some(self.make_token(Token::True, position)))},
            'f' => {self.read_literal("false")?; Ok(Some(self.make_token(Token::False, position)))},
            'n' => {self.read_literal("null")?; Ok(Some(self.make_token(Token::Null, position)))},
            c if c.is_ascii_digit() || c == '-' => {
                let number = self.read_number()?;
                Ok(Some(self.make_token(Token::Number(number), position)))
            },
            _ => Err(JsonError::Lexer(LexerError::UnexpectedCharacter{
                ch,
                position,
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
                        Some(other) => return Err(JsonError::Lexer(LexerError::InvalidEscapeCharacter{ch: other, position: self.current_positon()})),
                        None => return Err(JsonError::Lexer(LexerError::UnexpectedEndOfInput(self.current_positon())))
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
                _ => return Err(JsonError::Lexer(LexerError::InvalidUnicodeEscape(self.current_positon())))
            }
        }
        let code_point = u32::from_str_radix(&hex, 16)
                .map_err(|_| JsonError::Lexer(LexerError::InvalidUnicodeEscape(self.current_positon())))?;
        char::from_u32(code_point)
            .ok_or(JsonError::Lexer(LexerError::InvalidUnicodeEscape(self.current_positon())))
    }

    fn read_number(&mut self) -> Result<f64, JsonError> {
        let mut number = String::new();
        if self.peek() == Some('-') {
            number.push('-');
            self.next();
        }

        if let Some(ch) = self.next() {
            if ch == '0' {
                number.push(ch);
                match self.peek() {
                    Some(c) if c.is_ascii_digit() => return Err(JsonError::Lexer(LexerError::LeadingZero(self.current_positon()))),
                    _ => {}
                }
            } else if ch.is_ascii_digit() {
                number.push(ch);
            }
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
            return Err(JsonError::Lexer(LexerError::InvalidNumber(self.current_positon())))
        }
        number
            .parse::<f64>()
            .map_err(|_| JsonError::Lexer(LexerError::InvalidNumber(self.current_positon())))
    }

    fn read_literal(&mut self, expected: &str) -> Result<(), JsonError> {
        for expected_char in expected.chars() {
            match self.next() {
                Some(c) if c == expected_char => {},
                _ => return Err(JsonError::Lexer(LexerError::UnexpectedLiteral(self.current_positon()))),
            }
        }
        if let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                return Err(JsonError::Lexer(LexerError::UnexpectedLiteral(self.current_positon())))
            }
        }
        Ok(())
    }
}