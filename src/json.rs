use core::str;
#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Json>),
    Object(std::collections::HashMap<String, Json>),
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
    character::complete::{self, char, multispace0, satisfy},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn json_string(input: &str) -> IResult<&str, Json> {
    let (rest, parsed) = delimited(
        char('"'),
        map(many0(satisfy(|c| c != '"')), |chars: Vec<char>| {
            chars.into_iter().collect::<String>()
        }),
        char('"'),
    )(input)?;
    Ok((rest, Json::String(parsed)))
}

fn json_bool(input: &str) -> IResult<&str, Json> {
    alt((
        map(tag("false"), |_| Json::Bool(false)),
        map(tag("true"), |_| Json::Bool(true)),
    ))(input)
}

fn json_number(input: &str) -> IResult<&str, Json> {
    let (rest, digits) = complete::i32(input)?;

    Ok((rest, Json::Number(digits)))
}

fn white_space(input: &str) -> IResult<&str, Json> {
    delimited(multispace0, json, multispace0)(input)
}

fn json_array(input: &str) -> IResult<&str, Json> {
    let (rest, parsed) = delimited(
        char('['),
        separated_list0(char(','), delimited(multispace0, json, multispace0)),
        char(']'),
    )(input)?;
    Ok((rest, Json::Array(parsed)))
}

fn convert_json_to_string(j: Json) -> String {
    let convert = match j {
        Json::String(s) => Some(s),
        _ => None,
    };

    convert.expect("i hate rust compiler")
}

fn entry(input: &str) -> IResult<&str, (String, Json)> {
    map(
        tuple((
            terminated(delimited(multispace0, json_string, multispace0), char(':')),
            delimited(multispace0, json, multispace0),
        )),
        |(key, value)| (convert_json_to_string(key), value),
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
