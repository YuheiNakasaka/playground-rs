#[derive(Debug)]
pub struct Lexer {
    input: String,
    scope: Scope,
}

#[derive(PartialEq, Debug)]
pub enum Scope {
    Global,
    Object,
    Array,
    Key,
    String,
    Number,
    Boolean,
    Null,
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

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            scope: Scope::Global,
        }
    }

    pub fn run(&mut self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut position: usize = 0;
        while self.input.len() > position {
            let char: char = self.input.chars().nth(position as usize).unwrap();
            if self.scope != Scope::String {
                if char == LEFT_BRACE {
                    self.scope = Scope::Object;
                    tokens.push(char.to_string());
                    position += 1;
                } else if char == RIGHT_BRACE {
                    self.scope = Scope::Global;
                    tokens.push(char.to_string());
                    position += 1;
                } else if char == LEFT_BRACKET {
                    self.scope = Scope::Array;
                    tokens.push(char.to_string());
                    position += 1;
                } else if char == RIGHT_BRACKET {
                    self.scope = Scope::Global;
                    tokens.push(char.to_string());
                    position += 1;
                } else if char == COLLON {
                    self.scope = Scope::Global;
                    tokens.push(char.to_string());
                    position += 1;
                } else if char == COMMA {
                    self.scope = Scope::Global;
                    tokens.push(char.to_string());
                    position += 1;
                } else if char == INITIAL_TRUE {
                    self.scope = Scope::Boolean;
                    position += 3;
                    tokens.push("true".to_string());
                } else if char == INITIAL_FALSE {
                    self.scope = Scope::Boolean;
                    position += 4;
                    tokens.push("false".to_string());
                } else if char == INITIAL_NULL {
                    self.scope = Scope::Null;
                    position += 3;
                    tokens.push("null".to_string());
                } else if char.to_string().parse::<f64>().is_ok() {
                    self.scope = Scope::Number;
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
                    tokens.push(tmp_string);
                } else if char == DOUBLE_QUOTE {
                    self.scope = Scope::String;
                    tokens.push(char.to_string());
                    position += 1;
                } else {
                    position += 1;
                }
            } else {
                if char == DOUBLE_QUOTE {
                    self.scope = Scope::Global;
                    tokens.push(char.to_string());
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
                    tokens.push(tmp_string);
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
        assert_eq!(ret, vec!["{", "}"]);
    }

    #[test]
    fn blank_object() {
        let json = r#"{
        }"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["{", "}"]);
    }

    #[test]
    fn empty_array() {
        let json = r#"[]"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["[", "]"]);
    }

    #[test]
    fn filled_array() {
        let json = r#"[1,2]"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["[", "1", ",", "2", "]"]);
    }

    #[test]
    fn number_value() {
        let json = r#"1"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["1"]);
    }

    #[test]
    fn numbers_value() {
        let json = r#"123"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["123"]);
    }

    #[test]
    fn float_number_value() {
        let json = r#"3.14"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["3.14"]);
    }

    #[test]
    fn binint_number_value() {
        let json = r#"6.62607e-34"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["6.62607e-34"]);
    }

    #[test]
    fn true_value() {
        let json = r#"true"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["true"]);
    }

    #[test]
    fn false_value() {
        let json = r#"false"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["false"]);
    }

    #[test]
    fn null_value() {
        let json = r#"null"#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["null"]);
    }

    #[test]
    fn string_value() {
        let json = r#""abc""#;
        let ret = Lexer::new(json).run();
        assert_eq!(ret, vec!["\"", "abc", "\""]);
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
        assert_eq!(
            ret,
            vec![
                "{",
                "\"",
                "name",
                "\"",
                ":",
                "\"",
                "Yuhei Nakasaka",
                "\"",
                ",",
                "\"",
                "age",
                "\"",
                ":",
                "32",
                ",",
                "\"",
                "is_programmer",
                "\"",
                ":",
                "true",
                ",",
                "\"",
                "is_married",
                "\"",
                ":",
                "false",
                ",",
                "\"",
                "sex",
                "\"",
                ":",
                "null",
                ",",
                "\"",
                "x",
                "\"",
                ":",
                "[",
                "1.2",
                ",",
                "2",
                ",",
                "3.5",
                "]",
                ",",
                "\"",
                "y",
                "\"",
                ":",
                "[",
                "\"",
                "a",
                "\"",
                ",",
                "\"",
                "b",
                "\"",
                "]",
                ",",
                "\"",
                "z",
                "\"",
                ":",
                "{",
                "\"",
                "a",
                "\"",
                ":",
                "1",
                ",",
                "\"",
                "b",
                "\"",
                ":",
                "{",
                "\"",
                "c",
                "\"",
                ":",
                "2.5",
                "}",
                "}",
                ",",
                "}"
            ]
        );
    }

    // TODO: escaped string is not supported yet
    // #[test]
    // fn escaped_string_value() {
    //     let json = r#""ab\"c""#;
    //     let ret = Lexer::new(json).run();
    //     assert_eq!(ret, vec!["\"", "ab\"c", "\""]);
    // }
}
