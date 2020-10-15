#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
    height: u8,
}

#[test]
fn it_should_match_serde_output() {
    for input in &[
        r#"{"age":21,"height":187}"#,
        r#" { "age": 42, "height": 187 } "#,
    ] {
        let serde_output: Person =
            serde_json::from_str(input).expect(&format!("serde failed to parse: '{}'", input));
        let output =
            Person::from_json_str(input).expect(&format!("rigid failed to parse: '{}'", input));
        assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
    }
}
