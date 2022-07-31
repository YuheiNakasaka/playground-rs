use crate::lexer::Token;
use crate::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
}

impl ParserError {
    pub fn new(message: &str) -> ParserError {
        ParserError {
            message: message.to_string(),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Value, ParserError> {
        let token = self.tokens.get(self.position).unwrap().clone();
        let result = match token {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::String(s) => {
                self.position += 1;
                Ok(Value::String(s.clone()))
            }
            Token::Number(s) => {
                self.position += 1;
                Ok(Value::Number(s.clone()))
            }
            Token::True => {
                self.position += 1;
                Ok(Value::Boolean(true))
            }
            Token::False => {
                self.position += 1;
                Ok(Value::Boolean(false))
            }
            Token::Null => {
                self.position += 1;
                Ok(Value::Null)
            }
            _ => Err(ParserError::new(&format!(
                "Unacceptable token: {:?}",
                token
            ))),
        };
        result
    }

    fn parse_object(&mut self) -> Result<Value, ParserError> {
        self.position += 1;
        let mut object = BTreeMap::new();
        if self.tokens.get(self.position).unwrap() == &Token::RightBrace {
            return Ok(Value::Object(object));
        }

        loop {
            let key_token = self.tokens.get(self.position).unwrap().clone();
            self.position += 1;
            let colon_token = self.tokens.get(self.position).unwrap().clone();
            self.position += 1;

            match (key_token, colon_token) {
                (Token::String(key), Token::Colon) => {
                    object.insert(key.to_string(), self.parse().unwrap());
                }
                _ => return Err(ParserError::new("Expected key")),
            }

            let next_token = self.tokens.get(self.position).unwrap().clone();
            match next_token {
                Token::RightBrace => {
                    self.position += 1;
                    return Ok(Value::Object(object));
                }
                Token::Comma => {
                    self.position += 1;
                    continue;
                }
                _ => return Err(ParserError::new("Expected comma or right brace")),
            }
        }
    }

    fn parse_array(&mut self) -> Result<Value, ParserError> {
        self.position += 1;
        if self.tokens.get(self.position).unwrap() == &Token::RightBracket {
            return Ok(Value::Array(vec![]));
        }
        let mut array = vec![];
        loop {
            array.push(self.parse()?);
            let next_token = self.tokens.get(self.position).unwrap().clone();
            match next_token {
                Token::RightBracket => {
                    self.position += 1;
                    return Ok(Value::Array(array));
                }
                Token::Comma => {
                    self.position += 1;
                    continue;
                }
                _ => return Err(ParserError::new("Expected comma or right bracket")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_token() {
        let tokens = vec![Token::String("abc".to_string())];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::String("abc".to_string());
        assert_eq!(ret, expected);
    }

    #[test]
    fn number_token() {
        let tokens = vec![Token::Number(1.0)];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Number(1.0);
        assert_eq!(ret, expected);
    }

    #[test]
    fn true_token() {
        let tokens = vec![Token::True];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Boolean(true);
        assert_eq!(ret, expected);
    }

    #[test]
    fn false_token() {
        let tokens = vec![Token::False];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Boolean(false);
        assert_eq!(ret, expected);
    }

    #[test]
    fn null_token() {
        let tokens = vec![Token::Null];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Null;
        assert_eq!(ret, expected);
    }

    #[test]
    fn empty_object() {
        let tokens = vec![Token::LeftBrace, Token::RightBrace];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Object(BTreeMap::new());
        assert_eq!(ret, expected);
    }

    // {"a": "A"}
    #[test]
    fn simple_object() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("a".to_string()),
            Token::Colon,
            Token::String("A".to_string()),
            Token::RightBrace,
        ];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let mut object = BTreeMap::new();
        object.insert("a".to_string(), Value::String("A".to_string()));
        let expected = Value::Object(object);
        assert_eq!(ret, expected);
    }

    // {"a": {"b": "B", "c": "C"}}
    #[test]
    fn nested_object() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("a".to_string()),
            Token::Colon,
            Token::LeftBrace,
            Token::String("b".to_string()),
            Token::Colon,
            Token::String("B".to_string()),
            Token::Comma,
            Token::String("c".to_string()),
            Token::Colon,
            Token::String("C".to_string()),
            Token::RightBrace,
            Token::RightBrace,
        ];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let mut object = BTreeMap::new();
        let mut inner_object = BTreeMap::new();
        inner_object.insert("b".to_string(), Value::String("B".to_string()));
        inner_object.insert("c".to_string(), Value::String("C".to_string()));
        object.insert("a".to_string(), Value::Object(inner_object));
        let expected = Value::Object(object);
        assert_eq!(ret, expected);
    }

    #[test]
    fn empty_array() {
        let tokens = vec![Token::LeftBracket, Token::RightBracket];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Array(vec![]);
        assert_eq!(ret, expected);
    }

    // [1, "a", null, true, false]
    #[test]
    fn simple_array() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::String("a".to_string()),
            Token::Comma,
            Token::Null,
            Token::Comma,
            Token::True,
            Token::Comma,
            Token::False,
            Token::RightBracket,
        ];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Array(vec![
            Value::Number(1.0),
            Value::String("a".to_string()),
            Value::Null,
            Value::Boolean(true),
            Value::Boolean(false),
        ]);
        assert_eq!(ret, expected);
    }

    // [1, [2, [3]]]
    #[test]
    fn nested_array() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::LeftBracket,
            Token::Number(2.0),
            Token::Comma,
            Token::LeftBracket,
            Token::Number(3.0),
            Token::RightBracket,
            Token::RightBracket,
            Token::RightBracket,
        ];
        let mut parser = Parser::new(tokens);
        let ret = parser.parse().unwrap();
        let expected = Value::Array(vec![
            Value::Number(1.0),
            Value::Array(vec![
                Value::Number(2.0),
                Value::Array(vec![Value::Number(3.0)]),
            ]),
        ]);
        assert_eq!(ret, expected);
    }
}
