use std::collections::HashMap;
use crate::{JsonError, JsonValue, Position};
use crate::lexer::{Lexer, SpannedToken, Token};

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
    tokens: Vec<SpannedToken>,
    position: usize,
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

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }
    fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        if self.peek().is_some() {
            let token = self.peek().unwrap();
            return Err(JsonError::Parser(ParserError::UnexpectedToken(token.position)))
        }
        Ok(value)
    }

    fn peek(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<SpannedToken> {
        let token = self.peek()?.clone();
        self.position += 1;
        Some(token)
    }
    fn expect(&mut self, expected: Token) -> Result<(), JsonError> {
        match self.next() {
            Some(token) if token.token == expected => Ok(()),
            Some(token) => Err(JsonError::Parser(ParserError::UnexpectedToken(token.position))),
            None => Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
        }
    }

    fn current_token(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.position)
    }

    fn previous_token(&self) -> Option<&SpannedToken> {
        if self.position == 0 {
            None
        } else {
            self.tokens.get(self.position - 1)
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match self.current_token() {
            Some(token) => match &token.token {
            Token::Null => {
                self.next();
                Ok(JsonValue::Null)
            },
            Token::True => {
                self.next();
                Ok(JsonValue::Bool(true))
            },
            Token::False => {
                self.next();
                Ok(JsonValue::Bool(false))
            },
            Token::Number(_) => {
                let token = self.next().unwrap();
                match token.token {
                    Token::Number(n) => Ok(JsonValue::Number(n)),
                    _ => Err(JsonError::Parser(ParserError::InvalidNumber(token.position)))
                }
            },
            Token::String(_) => {
                let token = self.next().unwrap();
                match token.token {
                    Token::String(s) => Ok(JsonValue::String(s)),
                    _ => unreachable!()
                }
            },
            Token::LeftBracket => self.parse_array(),
            Token::LeftBrace => self.parse_object(),
            _ => Err(JsonError::Parser(ParserError::UnexpectedToken(token.position))),
            },
            None => Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
        }
    }

    fn eof_position(&self) -> Position {
        match self.previous_token() {
            Some(token) => token.position,
            None => Position {
                line: 1,
                column: 1
            }
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.expect(Token::LeftBracket)?;
        let mut array: Vec<JsonValue> = Vec::new();

        loop {
            match self.current_token() {
                Some(token) => {
                    match &token.token {
                        Token::RightBracket => {
                            self.next();
                            break;
                        },
                        _ => {
                            let value = self.parse_value()?;
                            array.push(value);
                            match self.current_token() {
                                Some(token) => match &token.token {
                                    Token::Comma => {
                                        self.next();
                                        match self.current_token() {
                                            Some(token) => match &token.token {
                                                Token::RightBracket => return Err(JsonError::Parser(ParserError::TrailingComma(token.position))),
                                                _ => continue
                                            },
                                            None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position()))),
                                        }
                                    }
                                    Token::RightBracket => continue,
                                    _ => return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBracket(token.position)))
                                },
                                None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
                            }
                        }
                    }
                },
                None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
            }
        }
        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.expect(Token::LeftBrace)?;
        let mut object = HashMap::new();

        loop {
            match self.current_token() {
                Some(token) => {
                    match &token.token {
                        Token::RightBrace => {
                            self.next();
                            break;
                        },
                        Token::String(_) => {
                            let token = self.next().unwrap();
                            let key = match token.token {
                                Token::String(s) => s,
                                _ => return Err(JsonError::Parser(ParserError::ExpectedStringKey(token.position))),
                            };
                            self.expect(Token::Colon)?;
                            let value = self.parse_value()?;
                            object.insert(key, value);
                            match self.current_token() {
                                Some(token) => {
                                    match &token.token {
                                        Token::Comma => {
                                            self.next();
                                            match self.current_token() {
                                                Some(token) => match &token.token {
                                                    Token::RightBrace => return Err(JsonError::Parser(ParserError::TrailingComma(token.position))),
                                                    _ => continue
                                                },
                                                None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position()))),
                                            }
                                        },
                                        Token::RightBrace => continue,
                                        _ => return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBrace(token.position)))
                                    }
                                },
                                None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
                            }
                        },
                        _token => return Err(JsonError::Parser(ParserError::UnexpectedToken(token.position)))
                    }
                }
                None => return Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
            }
        }
        Ok(JsonValue::Object(object))
    }
}
