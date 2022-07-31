# scrach_json_parser

## Example

```rs
use scrach_json_parser::lexer::Lexer;
use scrach_json_parser::parser::Parser;

fn main() {
    let json = r#"
  {
    "a": "A",
    "b": {
      "c": "C",
      "d": "D"
    }
  }
  "#;
    let result = Parser::new(Lexer::new(json).run()).parse().unwrap();
    println!("{:?}", result);
    //=> Object({"a": String("A"), "b": Object({"c": String("C"), "d": String("D")})})
}
```
