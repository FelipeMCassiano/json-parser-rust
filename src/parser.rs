use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited, IResult};

pub fn curly_braces(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}

#[test]
fn parens_test() {
    assert_eq!(curly_braces("{felipe : oi}"), Ok(("", "felipe : oi")))
}

// parse to tranform string to i32
// tag == token
// alt == any
