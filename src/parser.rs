use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use crate::{JsonError, JsonValue, Position};
use crate::lexer::{Lexer, SpannedToken, Token};

pub fn parse_from_str(input: &str) -> Result<JsonValue, JsonError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    parser.parse()
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<SpannedToken>,
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
            ParserError::InvalidNumber(pos ) => write!(f, "Invalid number at {pos}"),
            ParserError::UnterminatedString(pos) => write!(f, "Unterminated string at {pos}"),
            ParserError::TrailingComma(pos) => write!(f, "Trailing comma at {pos}"),
            ParserError::ExpectedComma(pos) => write!(f, "Expected comma at {pos}"),
            ParserError::ExpectedColon(pos) => write!(f, "Expected colon at {pos}"),
            ParserError::ExpectedCommaOrRightBracket(pos) => write!(f, "Expected comma or right bracket at {pos}"),
            ParserError::ExpectedCommaOrRightBrace(pos) => write!(f, "Expected comma or right brace at {pos}"),
            ParserError::ExpectedStringKey(pos) => write!(f, "Expected a string key at {pos}")
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, JsonError> {
        let current = lexer.next_token()?;
        Ok(Self {
            lexer,
            current
        })
    }
    fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        if self.current_token().is_some() {
            let token = self.current_token().unwrap();
            return Err(JsonError::Parser(ParserError::UnexpectedToken(token.position)))
        }
        Ok(value)
    }

    fn advance(&mut self) -> Result<Option<SpannedToken>, JsonError> {
        let token = self.current.clone();
        self.current = self.lexer.next_token()?;
        Ok(token)
    }

    fn expect(&mut self, expected: Token) -> Result<(), JsonError> {
        if self.match_token(&expected)? {
            Ok(())
        } else {
            match self.current_token() {
                Some(token) => Err(JsonError::Parser(ParserError::UnexpectedToken(token.position))),
                None => Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
            }
        }
    }


    fn check(&self, token: &Token) -> bool {
        self.current.as_ref().map(|t| &t.token) == Some(token)
    }

    fn match_token(&mut self, token: &Token) -> Result<bool, JsonError> {
        if self.check(token) {
            self.advance()?;
            return Ok(true)
        }
        Ok(false)
    }

    fn current_token(&self) -> Option<&SpannedToken> {
        self.current.as_ref()
    }

    fn eof_position(&self) -> Position {
        self.lexer.current_position()
    }

    fn current_position(&self) -> Position {
        match self.current_token() {
            Some(token) => token.position,
            None => self.eof_position()
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        let token = match self.current_token() {
            Some(t) => t.clone(),
            None => {
                return Err(JsonError::Parser(
                    ParserError::UnexpectedEndOfInput(self.current_position()),
                ))
            }
        };

        match token.token {
            Token::Null => {
                self.advance()?;
                Ok(JsonValue::Null)
            },
            Token::True => {
                self.advance()?;
                Ok(JsonValue::Bool(true))
            },
            Token::False => {
                self.advance()?;
                Ok(JsonValue::Bool(false))
            },
            Token::Number(n) => {
                self.advance()?;
                Ok(JsonValue::Number(n))
            },
            Token::String(s) => {
                self.advance()?;
                Ok(JsonValue::String(s))
            },
            Token::LeftBracket => self.parse_array(),
            Token::LeftBrace => self.parse_object(),
            _ => Err(JsonError::Parser(ParserError::UnexpectedToken(token.position))),
            }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        // Consume opening bracket
        self.expect(Token::LeftBracket)?;
        let mut array: Vec<JsonValue> = Vec::new();
        // Handle empty array early
        if self.match_token(&Token::RightBracket)? {
            return Ok(JsonValue::Array(array));
        }

        loop {
            let value = self.parse_value()?;
            array.push(value);
            // If comma
            if self.match_token(&Token::Comma)? {
                // Check trailing comma
                if self.check(&Token::RightBracket) {
                    return Err(JsonError::Parser(ParserError::TrailingComma(self.current_position())))
                }
                continue;
            }
            // If closing bracket
            if self.match_token(&Token::RightBracket)? {
                break;
            }
            // Otherwise Error
            return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBracket(self.current_position())))
        }
        // return array
        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        // Consume opening brace
        self.expect(Token::LeftBrace)?;
        let mut object = BTreeMap::new();
        // Handle empty object early
        if self.match_token(&Token::RightBrace)? {
            return Ok(JsonValue::Object(object))
        }

        loop {
            let key_token = self.advance()?
                .ok_or_else(|| {
                    JsonError::Parser(
                        ParserError::UnexpectedEndOfInput(
                            self.current_position()
                        )
                    )
                })?;

            return match key_token.token {
                Token::String(s) => {
                    // Consume colon
                    self.expect(Token::Colon)?;
                    // Parse value
                    let value = self.parse_value()?;
                    object.insert(s, value);
                    // If comma
                    if self.match_token(&Token::Comma)? {
                        // Check trailing comma
                        if self.check(&Token::RightBrace) {
                            return Err(JsonError::Parser(ParserError::TrailingComma(self.current_position())))
                        }
                        continue;
                    }
                    // If closing brace
                    if self.match_token(&Token::RightBrace)? {
                        break;
                    }
                    // Otherwise error
                    Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBrace(self.current_position())))
                }
                _ => Err(JsonError::Parser(ParserError::ExpectedStringKey(key_token.position)))
            }
        }
        Ok(JsonValue::Object(object))
    }
}
