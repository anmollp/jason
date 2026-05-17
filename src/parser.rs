use std::collections::HashMap;
use crate::{JsonError, JsonValue};
use crate::lexer::{Lexer, Token};

pub fn parse_from_str(input: &str) -> Result<JsonValue, JsonError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token()? {
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEndOfInput,
    InvalidNumber,
    UnterminatedString,
    TrailingComma,
    ExpectedComma,
    ExpectedColon,
    ExpectedCommaOrRightBracket,
    ExpectedCommaOrRightBrace,
    ExpectedStringKey
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }
    fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        if self.peek().is_some() {
            return Err(JsonError::Parser(ParserError::UnexpectedToken));
        }
        Ok(value)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.peek()?.clone();
        self.position += 1;
        Some(token)
    }
    fn expect(&mut self, expected: Token) -> Result<(), JsonError> {
        match self.next() {
            Some(token) if token == expected => Ok(()),
            Some(_token) => Err(JsonError::Parser(ParserError::UnexpectedToken)),
            None => Err(JsonError::Parser(ParserError::UnexpectedEndOfInput))
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match self.peek() {
            Some(Token::Null) => {
                self.next();
                Ok(JsonValue::Null)
            },
            Some(Token::True) => {
                self.next();
                Ok(JsonValue::Bool(true))
            },
            Some(Token::False) => {
                self.next();
                Ok(JsonValue::Bool(false))
            },
            Some(Token::Number(_)) => match self.next() {
                Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
                _ => Err(JsonError::Parser(ParserError::InvalidNumber))
            },
            Some(Token::String(_)) => match self.next() {
                Some(Token::String(s)) => Ok(JsonValue::String(s)),
                _ => Err(JsonError::Parser(ParserError::UnterminatedString))
            },
            Some(Token::LeftBracket) => self.parse_array(),
            Some(Token::LeftBrace) => self.parse_object(),
            _ => Err(JsonError::Parser(ParserError::UnexpectedToken))
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.expect(Token::LeftBracket)?;
        let mut array: Vec<JsonValue> = Vec::new();

        loop {
            match self.peek() {
                Some(Token::RightBracket) => {
                    self.next();
                    break;
                }
                _ => {
                let value = self.parse_value()?;
                array.push(value);
                    match self.peek() {
                        Some(Token::Comma) => {
                            self.next();
                            if matches!(self.peek(), Some(Token::RightBracket)) {
                                return Err(JsonError::Parser(ParserError::TrailingComma));
                            }
                        }
                        Some(Token::RightBracket) => {}
                        _ => {  return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBracket)) }
                    }
                }
            }
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.expect(Token::LeftBrace)?;
        let mut object = HashMap::new();

        loop {
            match self.peek() {
                Some(Token::RightBrace) => {
                    self.next();
                    break;
                }
                Some(Token::String(_)) => {
                    let key = match self.next() {
                        Some(Token::String(s)) => s,
                        _ => unreachable!()
                    };
                    self.expect(Token::Colon)?;
                    let value = self.parse_value()?;
                    object.insert(key, value);
                    match self.peek() {
                        Some(Token::Comma) => {
                            self.next();
                            if matches!(self.peek(), Some(Token::RightBrace)) {
                                return Err(JsonError::Parser(ParserError::TrailingComma))
                            }
                        },
                        Some(Token::RightBrace) => {}
                        _ => return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBrace))
                    }
                }
                _ => return Err(JsonError::Parser(ParserError::ExpectedStringKey))
            }
        }
        Ok(JsonValue::Object(object))
    }
}
