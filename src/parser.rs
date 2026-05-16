use std::collections::HashMap;
use crate::JsonValue;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn parse(&mut self) -> Result<JsonValue, String> {
        self.parse_value()
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
            Some(Token::Null) => Ok(JsonValue::Null),
            Some(Token::True) => Ok(JsonValue::Bool(true)),
            Some(Token::False) => Ok(JsonValue::Bool(false)),
            Some(Token::Number(_)) => match self.next() {
                Token::Number(n) => Ok(JsonValue::Number(n))
            },
            Some(Token::String(_)) => match self.next() {
                Token::String(s) => Ok(JsonValue::String(s))
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
                        Some(Token::Comma) => {self.next();}
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
                        Some(Token::Comma) => {self.next();},
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
