#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct PrimitiveTypes {
    _bool: bool,
    _string: String,
    _u8: u8,
    _u16: u16,
    //_char: char,
    //_f32: f32,
    //_f64: f64,
    //_i16: i16,
    //_i32: i32,
    //_i64: i64,
    //_i8: i8,
    //_isize: isize,
    //_str: &str,
    //_u32: u32,
    //_u64: u64,
    //_usize: usize,
    // array
    // slice
    // tuple
}

#[test]
fn it_should_match_serde_output() {
    let input = r#"{"_bool": true, "_string": "foobar", "_u8": 42, "_u16": 42 }"#;
    let serde_output: PrimitiveTypes = serde_json::from_str(input)
        .unwrap_or_else(|err| panic!("serde failed to parse `{}` with error {}", input, err));
    let output = PrimitiveTypes::from_json(input)
        .unwrap_or_else(|_| panic!("rigid failed to parse `{}`", input));
    assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
}
