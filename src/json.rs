use nom::Parser;

pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(std::collections::HashMap<String, Json>),
}

pub fn json<T>(input: &str) -> impl Parser<Json, T, T> {}
