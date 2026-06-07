use crate::error::{JsonError, Position};
use crate::lexer::LexerError::UnterminatedString;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

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
    Null,
}

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub position: Position,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    byte_pos: usize,
    line: usize,
    column: usize,
}

enum JsonString<'a> {
    Borrowed(&'a str),
    Owned(String),
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter { ch: char, position: Position },
    UnterminatedString(Position),
    InvalidNumber(Position),
    UnexpectedLiteral(Position),
    UnterminatedNumber(Position),
    InvalidUnicodeEscape(Position),
    UnexpectedEndOfInput(Position),
    InvalidEscapeCharacter { ch: char, position: Position },
    LeadingZero(Position),
}

impl std::error::Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedEndOfInput(pos) => write!(f, "Unexpected end of input at {pos}"),
            LexerError::UnexpectedCharacter { ch, position } => {
                write!(f, "Unexpected character '{ch}' at {position}")
            }
            LexerError::InvalidNumber(pos) => write!(f, "Invalid number at {pos}"),
            LexerError::InvalidUnicodeEscape(pos) => write!(f, "Invalid unicode escape at {pos}"),
            LexerError::UnexpectedLiteral(pos) => write!(f, "Unexpected literal at {pos}"),
            LexerError::UnterminatedNumber(pos) => write!(f, "Unterminated number at {pos}"),
            LexerError::UnterminatedString(pos) => write!(f, "Unterminated string at {pos}"),
            LexerError::InvalidEscapeCharacter { ch, position } => {
                write!(f, "Invalid escape character '{ch}' at {position}")
            }
            LexerError::LeadingZero(pos) => write!(f, "Leading zero at {pos}"),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars().peekable(),
            byte_pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub(crate) fn current_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.byte_pos += ch.len_utf8();
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
                self.next();
            } else {
                break;
            }
        }
    }

    fn make_token(&self, token: Token, position: Position) -> SpannedToken {
        SpannedToken { token, position }
    }

    pub fn next_token(&mut self) -> Result<Option<SpannedToken>, JsonError> {
        self.skip_whitespace();
        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        let position = self.current_position();

        match ch {
            '{' => {
                self.next();
                Ok(Some(self.make_token(Token::LeftBrace, position)))
            }
            '}' => {
                self.next();
                Ok(Some(self.make_token(Token::RightBrace, position)))
            }
            '[' => {
                self.next();
                Ok(Some(self.make_token(Token::LeftBracket, position)))
            }
            ']' => {
                self.next();
                Ok(Some(self.make_token(Token::RightBracket, position)))
            }
            ':' => {
                self.next();
                Ok(Some(self.make_token(Token::Colon, position)))
            }
            ',' => {
                self.next();
                Ok(Some(self.make_token(Token::Comma, position)))
            }
            '"' => {
                let string = self.read_string()?;
                let owned = match string {
                    JsonString::Borrowed(s) => s.to_string(),
                    JsonString::Owned(s) => s,
                };
                Ok(Some(self.make_token(Token::String(owned), position)))
            }
            't' => {
                self.read_literal("true")?;
                Ok(Some(self.make_token(Token::True, position)))
            }
            'f' => {
                self.read_literal("false")?;
                Ok(Some(self.make_token(Token::False, position)))
            }
            'n' => {
                self.read_literal("null")?;
                Ok(Some(self.make_token(Token::Null, position)))
            }
            c if c.is_ascii_digit() || c == '-' => {
                let number = self.read_number()?;
                Ok(Some(self.make_token(Token::Number(number), position)))
            }
            _ => Err(JsonError::Lexer(LexerError::UnexpectedCharacter {
                ch,
                position,
            })),
        }
    }

    fn read_escape(&mut self) -> Result<char, JsonError> {
        match self.next() {
            Some('n') => Ok('\n'),
            Some('t') => Ok('\t'),
            Some('\\') => Ok('\\'),
            Some('"') => Ok('"'),
            Some('u') => self.read_unicode_escape(),
            Some(other) => Err(JsonError::Lexer(LexerError::InvalidEscapeCharacter {
                ch: other,
                position: self.current_position(),
            })),
            None => Err(JsonError::Lexer(LexerError::UnexpectedEndOfInput(
                self.current_position(),
            ))),
        }
    }

    fn read_string(&mut self) -> Result<JsonString<'a>, JsonError> {
        let start = self.current_position();
        // consume opening quote and set start position after it
        // caller guarantees the next character is '"'
        self.next()
            .ok_or(JsonError::Lexer(UnterminatedString(start)))?;
        let start_pos = self.byte_pos;

        loop {
            let ch = self
                .next()
                .ok_or(JsonError::Lexer(UnterminatedString(start)))?;
            match ch {
                '"' => {
                    let end_pos = self.byte_pos - ch.len_utf8();
                    return Ok(JsonString::Borrowed(&self.input[start_pos..end_pos]));
                }
                '\\' => {
                    // switch to slow path: capture prefix, consume escape and push it
                    let escape_start = self.byte_pos - '\\'.len_utf8();
                    let mut buf = self.input[start_pos..escape_start].to_string();
                    let escaped = self.read_escape()?;
                    buf.push(escaped);
                    return self.read_string_slow(buf, start);
                }
                _ => {}
            }
        }
    }

    fn read_string_slow(
        &mut self,
        mut buf: String,
        start: Position,
    ) -> Result<JsonString<'a>, JsonError> {
        loop {
            let ch = self
                .next()
                .ok_or(JsonError::Lexer(UnterminatedString(start)))?;
            match ch {
                '"' => return Ok(JsonString::Owned(buf)),
                '\\' => buf.push(self.read_escape()?),
                _ => buf.push(ch),
            }
        }
    }

    fn read_unicode_escape(&mut self) -> Result<char, JsonError> {
        let mut hex = String::new();
        for _ in 0..4 {
            match self.next() {
                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                _ => {
                    return Err(JsonError::Lexer(LexerError::InvalidUnicodeEscape(
                        self.current_position(),
                    )));
                }
            }
        }
        let code_point = u32::from_str_radix(&hex, 16).map_err(|_| {
            JsonError::Lexer(LexerError::InvalidUnicodeEscape(self.current_position()))
        })?;
        char::from_u32(code_point).ok_or(JsonError::Lexer(LexerError::InvalidUnicodeEscape(
            self.current_position(),
        )))
    }

    fn read_number(&mut self) -> Result<f64, JsonError> {
        let mut number = String::new();
        // optional sign
        if self.peek() == Some('-') {
            number.push(self.next().unwrap());
        }

        // integer part
        let mut digit_count = 0;
        if self.peek() == Some('0') {
            number.push(self.next().unwrap());
            digit_count += 1;

            // leading zero check
            if let Some(ch) = self.peek()
                && ch.is_ascii_digit()
            {
                return Err(JsonError::Lexer(LexerError::LeadingZero(
                    self.current_position(),
                )));
            }
        } else {
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    digit_count += 1;
                    number.push(self.next().unwrap());
                } else {
                    break;
                }
            }
        }

        if digit_count == 0 {
            return Err(JsonError::Lexer(LexerError::InvalidNumber(
                self.current_position(),
            )));
        }

        // fractional part
        if self.peek() == Some('.') {
            number.push(self.next().unwrap());

            let mut frac_digits = 0;
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    frac_digits += 1;
                    number.push(self.next().unwrap());
                } else {
                    break;
                }
            }

            if frac_digits == 0 {
                return Err(JsonError::Lexer(LexerError::InvalidNumber(
                    self.current_position(),
                )));
            }
        }

        // exponent part
        if let Some(ch) = self.peek()
            && (ch == 'e' || ch == 'E')
        {
            number.push(self.next().unwrap());

            // optional exponent sign
            if let Some(ch) = self.peek()
                && (ch == '+' || ch == '-')
            {
                number.push(self.next().unwrap());
            }

            let mut exponent_digits = 0;
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    exponent_digits += 1;
                    number.push(self.next().unwrap());
                } else {
                    break;
                }
            }

            if exponent_digits == 0 {
                return Err(JsonError::Lexer(LexerError::InvalidNumber(
                    self.current_position(),
                )));
            }
        }

        number
            .parse::<f64>()
            .map_err(|_| JsonError::Lexer(LexerError::InvalidNumber(self.current_position())))
    }

    fn read_literal(&mut self, expected: &str) -> Result<(), JsonError> {
        for expected_char in expected.chars() {
            match self.next() {
                Some(c) if c == expected_char => {}
                _ => {
                    return Err(JsonError::Lexer(LexerError::UnexpectedLiteral(
                        self.current_position(),
                    )));
                }
            }
        }
        if let Some(ch) = self.peek()
            && (ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(JsonError::Lexer(LexerError::UnexpectedLiteral(
                self.current_position(),
            )));
        }
        Ok(())
    }
}

