use core::str;

use nom::{branch::alt, IResult};

pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(std::collections::HashMap<String, Json>),
}

pub fn json(input: &str) -> IResult<&str, Json> {
    alt((curly_braces, json_string, json_number, json_bool))(input)
}

use std::char;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{char, digit0, satisfy, space0},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::delimited,
};

pub fn curly_braces(input: &str) -> IResult<&str, Json> {
    let (input, strings) = delimited(char('{'), is_not("}"), char('}'))(input)?;
    Ok((input, Json::String(strings.to_string())))
}

#[test]
fn curly_braces_test() {
    assert_eq!(curly_braces("{felipe : oi}"), Ok(("", "felipe : oi")))
}

pub fn json_string(input: &str) -> IResult<&str, Json> {
    let (input, parsed) = delimited(
        char('"'),
        map(many0(satisfy(|c| c != '"')), |chars: Vec<char>| {
            chars.into_iter().collect::<String>()
        }),
        char('"'),
    )(input)?;

    Ok((input, Json::String(parsed)))
}

pub fn json_bool(input: &str) -> IResult<&str, Json> {
    alt((
        map(tag("false"), |_| Json::Bool(false)),
        map(tag("true"), |_| Json::Bool(true)),
    ))(input)
}

pub fn json_number(input: &str) -> IResult<&str, Json> {
    let (input, digits) = digit0(input)?;

    let parsed_number = digits.parse::<f64>().unwrap();
    Ok((input, Json::Number(parsed_number)))
}

pub fn json_array(input: &str) -> IResult<&str, Json> {
    let (input, parsed) = delimited(
        char('['),
        separated_list0(char(','), delimited(space0, json, space0)),
        char(']'),
    )(input)?;
    Ok((input, Json::Array(parsed)))
}

// parse to tranform string to i32
// tag == token
// alt == any
