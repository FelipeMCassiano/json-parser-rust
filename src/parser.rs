use std::char;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, satisfy},
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};
// todo: change the return type to str String
pub fn curly_braces(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}

#[test]
fn curly_braces_test() {
    assert_eq!(curly_braces("{felipe : oi}"), Ok(("", "felipe : oi")))
}

pub fn json_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(many0(satisfy(|c| c != '"')), |chars: Vec<char>| {
            chars.into_iter().collect::<String>()
        }),
        char('"'),
    )(input)
}

pub fn json_bool(input: &str) -> IResult<&str, bool> {
    alt((map(tag("false"), |_| false), map(tag("true"), |_| false)))(input)
}

// parse to tranform string to i32
// tag == token
// alt == any
