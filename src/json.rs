use core::str;

use nom::{branch::alt, character::complete::digit1, IResult};

use crate::parser::curly_braces;

pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(std::collections::HashMap<String, Json>),
}

pub fn json(input: &str) -> IResult<&str, &str> {
    alt((curly_braces, digit1))(input)
}
