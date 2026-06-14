use crate::error::{JsonError, Position};
use crate::lexer::{Lexer, SpannedToken, Token, JsonString};
use crate::value::JsonValue;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    peeked: Option<SpannedToken<'a>>,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Position),
    UnexpectedEndOfInput(Position),
    InvalidNumber(Position),
    UnterminatedString(Position),
    TrailingComma(Position),
    ExpectedComma(Position),
    ExpectedColon(Position),
    ExpectedCommaOrRightBracket(Position),
    ExpectedCommaOrRightBrace(Position),
    ExpectedStringKey(Position),
}

impl std::error::Error for ParserError {}
impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(pos) => write!(f, "Unexpected token at {pos}"),
            ParserError::UnexpectedEndOfInput(pos) => write!(f, "Unexpected EOF at {pos}"),
            ParserError::InvalidNumber(pos) => write!(f, "Invalid number at {pos}"),
            ParserError::UnterminatedString(pos) => write!(f, "Unterminated string at {pos}"),
            ParserError::TrailingComma(pos) => write!(f, "Trailing comma at {pos}"),
            ParserError::ExpectedComma(pos) => write!(f, "Expected comma at {pos}"),
            ParserError::ExpectedColon(pos) => write!(f, "Expected colon at {pos}"),
            ParserError::ExpectedCommaOrRightBracket(pos) => {
                write!(f, "Expected comma or right bracket at {pos}")
            }
            ParserError::ExpectedCommaOrRightBrace(pos) => {
                write!(f, "Expected comma or right brace at {pos}")
            }
            ParserError::ExpectedStringKey(pos) => write!(f, "Expected a string key at {pos}"),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self, JsonError> {
        Ok(Self {
            lexer,
            peeked: None,
        })
    }

    fn has_extra_tokens(&mut self) -> Result<bool, JsonError> {
        Ok(self.peek()?.is_some())
    }

    fn last_position(&mut self) -> Position {
        self.peeked.as_ref().map(|t| t.position).unwrap_or_default()
    }

    pub(crate) fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        if self.has_extra_tokens()? {
            let pos = self.last_position();
            return Err(JsonError::Parser(ParserError::UnexpectedToken(pos)));
        }
        Ok(value)
    }

    fn peek(&mut self) -> Result<Option<&SpannedToken<'_>>, JsonError> {
        if self.peeked.is_none() {
            self.peeked = self.lexer.next_token()?;
        }
        Ok(self.peeked.as_ref())
    }

    fn eat(&mut self, expected: Token) -> Result<bool, JsonError> {
        match self.peek()? {
            Some(t) if t.token == expected => {
                self.peeked.take();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn next(&mut self) -> Result<Option<SpannedToken<'_>>, JsonError> {
        if let Some(token) = self.peeked.take() {
            Ok(Some(token))
        } else {
            self.lexer.next_token()
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        let token = match self.next()? {
            Some(t) => t,
            None => {
                return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(
                    self.last_position(),
                )));
            }
        };

        match token.token {
            Token::Null => Ok(JsonValue::Null),
            Token::True => Ok(JsonValue::Bool(true)),
            Token::False => Ok(JsonValue::Bool(false)),

            Token::Number(n) => Ok(JsonValue::Number(n)),
            Token::String(s) => {
                let owned = match s {
                    JsonString::Borrowed(b) => b.to_string(),
                    JsonString::Owned(o) => o,
                };
                Ok(JsonValue::String(owned))
            }
            Token::LeftBracket => self.parse_array(),
            Token::LeftBrace => self.parse_object(),
            _ => Err(JsonError::Parser(ParserError::UnexpectedToken(
                token.position,
            ))),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        let mut array = Vec::new();

        if self.eat(Token::RightBracket)? {
            return Ok(JsonValue::Array(array));
        }

        loop {
            array.push(self.parse_value()?);

            if self.eat(Token::Comma)? {
                if self.eat(Token::RightBracket)? {
                    return Err(JsonError::Parser(ParserError::TrailingComma(
                        self.last_position(),
                    )));
                }
                continue;
            }

            if self.eat(Token::RightBracket)? {
                break;
            }

            return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBracket(
                self.last_position(),
            )));
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        let mut object = BTreeMap::new();
        // Handle empty object early
        if self.eat(Token::RightBrace)? {
            return Ok(JsonValue::Object(object));
        }

        loop {
            let key_token = match self.next()? {
                Some(t) => t,
                None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.last_position())))
            };

            let key = match key_token.token {
                Token::String(s) => match s {
                    JsonString::Borrowed(b) => b.to_string(),
                    JsonString::Owned(o) => o
                },
                _ => {
                    return Err(JsonError::Parser(ParserError::ExpectedStringKey(
                        key_token.position,
                    )));
                }
            };

            // expect ':'
            if !self.eat(Token::Colon)? {
                // If there's a different token where colon is expected, it's an unexpected token
                return if let Some(t) = self.peek()? {
                    Err(JsonError::Parser(ParserError::UnexpectedToken(t.position)))
                } else {
                    Err(JsonError::Parser(ParserError::ExpectedColon(
                        self.last_position(),
                    )))
                };
            }

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);
            // If comma
            if self.eat(Token::Comma)? {
                // Check trailing comma
                if self.eat(Token::RightBrace)? {
                    return Err(JsonError::Parser(ParserError::TrailingComma(
                        self.last_position(),
                    )));
                }
                continue;
            }

            // If closing brace
            if self.eat(Token::RightBrace)? {
                break;
            }

            return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBrace(
                self.last_position(),
            )));
        }
        Ok(JsonValue::Object(object))
    }
}
