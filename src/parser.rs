use std::collections::HashMap;
use crate::JsonValue;
use crate::lexer::{Lexer, Token};

pub fn parse_from_str(input: &str) -> Result<JsonValue, String> {
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

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }
    fn parse(&mut self) -> Result<JsonValue, String> {
        let value = self.parse_value()?;
        if self.peek().is_some() {
            return Err("Unexpected trailing tokens".to_string());
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
    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.next() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?} but received {:?}", expected, token)),
            None => Err("Unexpected end of input".to_string())
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
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
                _ => Err("Expected number".to_string())
            },
            Some(Token::String(_)) => match self.next() {
                Some(Token::String(s)) => Ok(JsonValue::String(s)),
                _ => Err("Expected string".to_string())
            },
            Some(Token::LeftBracket) => self.parse_array(),
            Some(Token::LeftBrace) => self.parse_object(),
            _ => Err("Unexpected json value".to_string())
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
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
                                return Err("Trailing commas are not allowed".to_string());
                            }
                        }
                        Some(Token::RightBracket) => {}
                        _ => {  return Err("Expected comma or ]".to_string()); }
                    }
                }
            }
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
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
                                return Err("Trailing commas are not allowed".to_string())
                            }
                        },
                        Some(Token::RightBrace) => {}
                        _ => return Err("Expected comma or }".to_string())
                    }
                }
                _ => return Err("Expected string key".to_string())
            }
        }
        Ok(JsonValue::Object(object))
    }
}
