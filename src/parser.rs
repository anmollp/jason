use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }
    fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        if self.current_token().is_some() {
            let token = self.current_token().unwrap();
            return Err(JsonError::Parser(ParserError::UnexpectedToken(token.position)))
        }
        Ok(value)
    }

    fn advance(&mut self) -> Option<SpannedToken> {
        let token = self.current_token()?.clone();
        self.position += 1;
        Some(token)
    }
    fn expect(&mut self, expected: Token) -> Result<(), JsonError> {
        match self.advance() {
            Some(token) if token.token == expected => Ok(()),
            Some(token) => Err(JsonError::Parser(ParserError::UnexpectedToken(token.position))),
            None => Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.eof_position())))
        }
    }

    fn check(&self, token: &Token) -> bool {
        match self.current_token() {
            Some(spanned_token) => spanned_token.token == *token,
            None => false
        }
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            return true
        }
        false
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

    fn eof_position(&self) -> Position {
        match self.previous_token() {
            Some(token) => token.position,
            None => Position {
                line: 1,
                column: 1
            }
        }
    }

    fn current_position(&self) -> Position {
        match self.current_token() {
            Some(token) => token.position,
            None => self.eof_position()
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match self.current_token() {
            Some(token) => match &token.token {
            Token::Null => {
                self.advance();
                Ok(JsonValue::Null)
            },
            Token::True => {
                self.advance();
                Ok(JsonValue::Bool(true))
            },
            Token::False => {
                self.advance();
                Ok(JsonValue::Bool(false))
            },
            Token::Number(_) => {
                let token = self.advance().unwrap();
                match token.token {
                    Token::Number(n) => Ok(JsonValue::Number(n)),
                    _ => Err(JsonError::Parser(ParserError::InvalidNumber(token.position)))
                }
            },
            Token::String(_) => {
                let token = self.advance().unwrap();
                match token.token {
                    Token::String(s) => Ok(JsonValue::String(s)),
                    _ => unreachable!()
                }
            },
            Token::LeftBracket => self.parse_array(),
            Token::LeftBrace => self.parse_object(),
            _ => Err(JsonError::Parser(ParserError::UnexpectedToken(token.position))),
            },
            None => Err(JsonError::Parser(ParserError::UnexpectedEndOfInput(self.current_position())))
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        // Consume opening bracket
        self.expect(Token::LeftBracket)?;
        let mut array: Vec<JsonValue> = Vec::new();
        // Handle empty array early
        if self.match_token(&Token::RightBracket) {
            return Ok(JsonValue::Array(array));
        }

        loop {
            let value = self.parse_value()?;
            array.push(value);
            // If comma
            if self.match_token(&Token::Comma) {
                // Check trailing comma
                if self.check(&Token::RightBracket) {
                    return Err(JsonError::Parser(ParserError::TrailingComma(self.current_position())))
                }
                continue;
            }
            // If closing bracket
            if self.match_token(&Token::RightBracket) {
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
        if self.match_token(&Token::RightBrace) {
            return Ok(JsonValue::Object(object))
        }

        loop {
            match self.current_token() {
                Some(token) => match &token.token {
                    Token::String(_) => {
                        let key_token = self.advance().unwrap();
                        // Parse key
                        let key = match key_token.token {
                            Token::String(s) => s,
                            _ => return Err(JsonError::Parser(ParserError::ExpectedStringKey(key_token.position))),
                        };
                        // Consume colon
                        self.expect(Token::Colon)?;
                        // Parse value
                        let value = self.parse_value()?;
                        object.insert(key, value);
                        // If comma
                        if self.match_token(&Token::Comma) {
                            // Check trailing comma
                            if self.check(&Token::RightBrace) {
                                return Err(JsonError::Parser(ParserError::TrailingComma(self.current_position())))
                            }
                            continue;
                        }
                        // If closing brace
                        if self.match_token(&Token::RightBrace) {
                            break;
                        }
                        // Otherwise error
                        return Err(JsonError::Parser(ParserError::ExpectedCommaOrRightBrace(self.current_position())))
                    }
                    _ => return Err(JsonError::Parser(ParserError::ExpectedStringKey(token.position)))
                },
                _ => return Err(JsonError::Parser(ParserError::UnexpectedToken(self.current_position())))
            }
        }
        Ok(JsonValue::Object(object))
    }
}
