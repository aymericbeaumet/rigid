#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
}

#[test]
fn it_should_match_serde_output() {
    for input in &[r#"{ "age": 43 }"#] {
        let output = Person::json_from_str(input).unwrap();
        let serde_output: Person = serde_json::from_str(input).unwrap();
        assert_eq!(output, serde_output);
    }
}

// TODO(test): do we want to detect duplicates?
// TODO(test): do we want to detect missing fields?
