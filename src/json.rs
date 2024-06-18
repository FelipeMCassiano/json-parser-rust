#![allow(dead_code)]
use core::{fmt, str};

pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(std::collections::HashMap<String, Json>),
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "json",)
    }
}

pub fn json(input: &str) -> IResult<&str, Json> {
    alt((
        json_null,
        json_bool,
        json_number,
        json_string,
        json_array,
        json_obj,
    ))(input)
}

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, multispace0, satisfy},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub fn json_string(input: &str) -> IResult<&str, Json> {
    let (rest, parsed) = delimited(
        char('"'),
        map(many0(satisfy(|c| c != '"')), |chars: Vec<char>| {
            chars.into_iter().collect::<String>()
        }),
        char('"'),
    )(input)?;

    Ok((rest, Json::String(parsed)))
}

pub fn json_bool(input: &str) -> IResult<&str, Json> {
    alt((
        map(tag("false"), |_| Json::Bool(false)),
        map(tag("true"), |_| Json::Bool(true)),
    ))(input)
}

pub fn json_number(input: &str) -> IResult<&str, Json> {
    let (rest, digits) = digit0(input)?;

    let parsed_number = digits.parse::<f64>().unwrap();
    Ok((rest, Json::Number(parsed_number)))
}

fn white_space(input: &str) -> IResult<&str, Json> {
    delimited(multispace0, json, multispace0)(input)
}

pub fn json_array(input: &str) -> IResult<&str, Json> {
    let (rest, parsed) = delimited(
        char('['),
        separated_list0(char(','), delimited(multispace0, json, multispace0)),
        char(']'),
    )(input)?;
    Ok((rest, Json::Array(parsed)))
}

fn entry(input: &str) -> IResult<&str, (String, Json)> {
    map(
        tuple((
            terminated(delimited(multispace0, json_string, multispace0), char(':')),
            delimited(multispace0, json, multispace0),
        )),
        |(key, value)| (key.to_string(), value),
    )(input)
}

fn json_obj(input: &str) -> IResult<&str, Json> {
    let (rest, parsed) = delimited(
        char('{'),
        separated_list0(char(','), delimited(multispace0, entry, multispace0)),
        char('}'),
    )(input)?;

    let obj = parsed.into_iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
    });

    Ok((rest, Json::Object(obj)))
}

fn json_null(input: &str) -> IResult<&str, Json> {
    map(tag("null"), |_| Json::Null)(input)
}
