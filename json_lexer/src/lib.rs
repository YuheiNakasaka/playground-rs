use std::collections::BTreeMap;

pub mod lexer;
pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(BTreeMap<String, Value>),
    Array(Vec<Value>),
}
