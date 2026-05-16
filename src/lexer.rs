#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(i64),
    True,
    False,
    Null
}

pub struct Lexer {
    chars: Vec<char>,
    position: usize
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer{
            chars: input.chars().collect(),
            position: 0
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
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

    fn next_token(&mut self) -> Result<Option<Token>, String> {
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
            c if c.is_ascii_digit() || c == '-' => Ok(Some(Token::Number(self.read_number()?))),
            _ => Err("Unexpected character".to_string()),
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut string_token = String::new();
        self.next();
        while let Some(ch) = self.next() {
            match ch {
                '"' => return Ok(string_token),
                _ => string_token.push(ch)
            };
        }
        Err("Unterminated String".to_string())
    }

    fn read_number(&mut self) -> Result<i64, String> {
        let mut number = String::new();
        if self.peek() == Some('-') {
            number.push('-');
            self.next();
        }

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.next();
            }
            else {
                break;
            }
        };
        number
            .parse::<i64>()
            .map_err(|_| "Invalid number".to_string())
    }
}