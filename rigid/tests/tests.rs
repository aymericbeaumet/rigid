#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Empty {}

#[test]
fn it_should_match_serde_output_for_struct_empty() {
    for input in &[r#"{}"#, r#" { } "#] {
        let serde_output: Empty =
            serde_json::from_str(input).expect(&format!("serde failed to parse: '{}'", input));
        let output =
            Empty::from_json_str(input).expect(&format!("rigid failed to parse: '{}'", input));
        assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct OneField {
    foo: u8,
}

#[test]
fn it_should_match_serde_output_for_struct_one_field() {
    for input in &[r#"{"foo":42}"#, r#" { "foo": 42 } "#] {
        let serde_output: OneField =
            serde_json::from_str(input).expect(&format!("serde failed to parse: '{}'", input));
        let output =
            OneField::from_json_str(input).expect(&format!("rigid failed to parse: '{}'", input));
        assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
    height: u8,
}

#[test]
fn it_should_match_serde_output_for_struct_person() {
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
