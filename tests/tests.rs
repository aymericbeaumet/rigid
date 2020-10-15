#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
}

#[test]
fn it_should_match_serde_output() {
    for input in &[r#"{"age":21}"#, r#" { "age": 42 } "#] {
        let output = Person::from_json_str(input).unwrap();
        let serde_output: Person = serde_json::from_str(input).unwrap();
        assert_eq!(output, serde_output);
    }
}
