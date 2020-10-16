#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct Empty {}

#[test]
fn it_should_match_serde_output_for_struct_empty() {
    for input in &[r#"{}"#, r#" { } "#] {
        let serde_output: Empty = serde_json::from_str(input)
            .unwrap_or_else(|err| panic!("serde failed to parse `{}` with error {}", input, err));
        let output = Empty::from_json(input)
            .unwrap_or_else(|err| panic!("rigid failed to parse `{}` with error {}", input, err));
        assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct OneField {
    alone: u8,
}

#[test]
fn it_should_match_serde_output_for_struct_one_field() {
    for input in &[r#"{"alone":42}"#, r#" { "alone": 42 } "#] {
        let serde_output: OneField = serde_json::from_str(input)
            .unwrap_or_else(|err| panic!("serde failed to parse `{}` with error {}", input, err));
        let output = OneField::from_json(input)
            .unwrap_or_else(|err| panic!("rigid failed to parse `{}` with error {}", input, err));
        assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct Person {
    height: u8,
    birth_year: u16,
}

#[test]
fn it_should_match_serde_output_for_struct_person() {
    for input in &[
        r#"{"height":187,"birth_year": 1992}"#,
        r#" { "height": 187 , "birth_year": 1992 } "#,
    ] {
        let serde_output: Person = serde_json::from_str(input)
            .unwrap_or_else(|err| panic!("serde failed to parse `{}` with error {}", input, err));
        let output = Person::from_json(input)
            .unwrap_or_else(|err| panic!("rigid failed to parse `{}` with error {}", input, err));
        assert_eq!(output, serde_output, "rigid's and serde's outputs differ");
    }
}
