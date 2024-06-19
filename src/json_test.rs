use crate::json::{json, Json};

#[cfg(test)]
#[test]
fn json_obj_test() {
    use std::collections::HashMap;
    let mut expected_map1 = HashMap::new();
    expected_map1.insert("key".to_string(), Json::String("value".to_string()));

    assert_eq!(
        json("{\"key\": \"value\"}"),
        Ok(("", Json::Object(expected_map1)))
    );

    let expected_map2 = Json::Object(
        [
            ("abc".to_string(), Json::Bool(true)),
            ("def".to_string(), Json::Number(1)),
            (
                "ghi".to_string(),
                Json::Array(vec![Json::Number(5), Json::Number(6)]),
            ),
        ]
        .into(),
    );

    assert_eq!(
        json("{\"abc\": true, \"def\": 1, \"ghi\": [5,6]}"),
        Ok(("", expected_map2))
    )
}

#[test]
fn json_null_test() {
    assert_eq!(json("null"), Ok(("", Json::Null)))
}

#[test]
fn json_number_test() {
    assert_eq!(json("123"), Ok(("", Json::Number(123))))
}

#[test]
fn json_bool_test() {
    assert_eq!(json("true"), Ok(("", Json::Bool(true))))
}

#[test]
fn json_string_test() {
    assert_eq!(json("\"abc\""), Ok(("", Json::String("abc".to_string()))))
}

#[test]
fn json_array_test() {
    assert_eq!(
        json("[1,2,3]"),
        Ok((
            "",
            Json::Array(vec![Json::Number(1), Json::Number(2), Json::Number(3)])
        ))
    );

    assert_eq!(
        json("[\"abc\"]"),
        Ok(("", Json::Array(vec![Json::String("abc".to_string())])))
    )
}
