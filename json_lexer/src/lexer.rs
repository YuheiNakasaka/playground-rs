#[derive(Debug, PartialEq)]
enum Scope {
    Global,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    DoubleQuote,
    String(String),
    Number(f64),
    True,
    False,
    Null,
    Eof,
}

const LEFT_BRACE: char = '{';
const RIGHT_BRACE: char = '}';
const LEFT_BRACKET: char = '[';
const RIGHT_BRACKET: char = ']';
const COLLON: char = ':';
const COMMA: char = ',';
const DOUBLE_QUOTE: char = '"';
const INITIAL_TRUE: char = 't';
const INITIAL_FALSE: char = 'f';
const INITIAL_NULL: char = 'n';
const BACK_SLASH: char = '\\';

#[derive(Debug)]
pub struct Lexer {
    input: String,
    scope: Scope,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            scope: Scope::Global,
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut position: usize = 0;
        while self.input.len() > position {
            let char: char = self.input.chars().nth(position as usize).unwrap();
            if self.scope != Scope::String {
                if char == LEFT_BRACE {
                    self.scope = Scope::Global;
                    tokens.push(Token::LeftBrace);
                    position += 1;
                } else if char == RIGHT_BRACE {
                    self.scope = Scope::Global;
                    tokens.push(Token::RightBrace);
                    position += 1;
                } else if char == LEFT_BRACKET {
                    self.scope = Scope::Global;
                    tokens.push(Token::LeftBracket);
                    position += 1;
                } else if char == RIGHT_BRACKET {
                    self.scope = Scope::Global;
                    tokens.push(Token::RightBracket);
                    position += 1;
                } else if char == COLLON {
                    self.scope = Scope::Global;
                    tokens.push(Token::Colon);
                    position += 1;
                } else if char == COMMA {
                    self.scope = Scope::Global;
                    tokens.push(Token::Comma);
                    position += 1;
                } else if char == INITIAL_TRUE {
                    self.scope = Scope::Global;
                    position += 3;
                    tokens.push(Token::True);
                } else if char == INITIAL_FALSE {
                    self.scope = Scope::Global;
                    position += 4;
                    tokens.push(Token::False);
                } else if char == INITIAL_NULL {
                    self.scope = Scope::Global;
                    position += 3;
                    tokens.push(Token::Null);
                } else if char.to_string().parse::<f64>().is_ok() {
                    self.scope = Scope::Global;
                    let mut tmp_string: String = String::new();
                    let mut tmp_char: char = char.clone();
                    while tmp_char != COMMA && tmp_char != RIGHT_BRACE && tmp_char != RIGHT_BRACKET
                    {
                        tmp_string.push(tmp_char);
                        position += 1;
                        match self.input.chars().nth(position as usize) {
                            Some(c) => tmp_char = c,
                            None => break,
                        }
                    }
                    let number = tmp_string.parse::<f64>().unwrap();
                    tokens.push(Token::Number(number));
                } else if char == DOUBLE_QUOTE {
                    self.scope = Scope::String;
                    position += 1;
                } else {
                    position += 1;
                }
            } else {
                if char == DOUBLE_QUOTE {
                    self.scope = Scope::Global;
                    position += 1;
                } else {
                    let mut tmp_string: String = String::new();
                    let mut prev_char: char = ' ';
                    let mut curr_char: char = char.clone();
                    while curr_char != DOUBLE_QUOTE
                        || (prev_char == BACK_SLASH && curr_char == DOUBLE_QUOTE)
                    {
                        tmp_string.push(curr_char);
                        position += 1;
                        prev_char = curr_char.clone();
                        match self.input.chars().nth(position as usize) {
                            Some(c) => curr_char = c,
                            None => break,
                        }
                    }
                    tokens.push(Token::String(tmp_string));
                }
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_object() {
        let json = r#"{     }"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::LeftBrace, Token::RightBrace];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn blank_object() {
        let json = r#"{
        }"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::LeftBrace, Token::RightBrace];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn empty_array() {
        let json = r#"[]"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::LeftBracket, Token::RightBracket];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn filled_array() {
        let json = r#"[1,2]"#;
        let ret = Lexer::new(json).run();
        let expected = vec![
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::RightBracket,
        ];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn number_value() {
        let json = r#"1"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::Number(1.0)];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn numbers_value() {
        let json = r#"123"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::Number(123.0)];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn float_number_value() {
        let json = r#"3.14"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::Number(3.14)];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn binint_number_value() {
        let json = r#"6.62607e-34"#;
        let ret = Lexer::new(json).run();
        println!("{:?}", ret);
        let expected = vec![Token::Number(6.62607e-34)];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn true_value() {
        let json = r#"true"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::True];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn false_value() {
        let json = r#"false"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::False];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn null_value() {
        let json = r#"null"#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::Null];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn string_value() {
        let json = r#""abc""#;
        let ret = Lexer::new(json).run();
        let expected = vec![Token::String(String::from("abc"))];
        assert!(ret.eq(&expected));
    }

    #[test]
    fn full_json() {
        let json = r#"{
          "name": "Yuhei Nakasaka",
          "age": 32,
          "is_programmer": true,
          "is_married": false,
          "sex": null,
          "x": [1.2, 2, 3.5],
          "y": ["a", "b"],
          "z": {"a": 1, "b": {"c": 2.5}},
        }"#;
        let ret = Lexer::new(json).run();
        let expected = vec![
            Token::LeftBrace,
            Token::String(String::from("name")),
            Token::Colon,
            Token::String(String::from("Yuhei Nakasaka")),
            Token::Comma,
            Token::String(String::from("age")),
            Token::Colon,
            Token::Number(32.0),
            Token::Comma,
            Token::String(String::from("is_programmer")),
            Token::Colon,
            Token::True,
            Token::Comma,
            Token::String(String::from("is_married")),
            Token::Colon,
            Token::False,
            Token::Comma,
            Token::String(String::from("sex")),
            Token::Colon,
            Token::Null,
            Token::Comma,
            Token::String(String::from("x")),
            Token::Colon,
            Token::LeftBracket,
            Token::Number(1.2),
            Token::Comma,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.5),
            Token::RightBracket,
            Token::Comma,
            Token::String(String::from("y")),
            Token::Colon,
            Token::LeftBracket,
            Token::String(String::from("a")),
            Token::Comma,
            Token::String(String::from("b")),
            Token::RightBracket,
            Token::Comma,
            Token::String(String::from("z")),
            Token::Colon,
            Token::LeftBrace,
            Token::String(String::from("a")),
            Token::Colon,
            Token::Number(1.0),
            Token::Comma,
            Token::String(String::from("b")),
            Token::Colon,
            Token::LeftBrace,
            Token::String(String::from("c")),
            Token::Colon,
            Token::Number(2.5),
            Token::RightBrace,
            Token::RightBrace,
            Token::Comma,
            Token::RightBrace,
        ];
        assert!(ret.eq(&expected));
    }

    // TODO: escaped string is not supported yet
    // #[test]
    // fn escaped_string_value() {
    //     let json = r#""ab\"c""#;
    //     let ret = Lexer::new(json).run();
    //     assert_eq!(ret, vec!["\"", "ab\"c", "\""]);
    // }
}
