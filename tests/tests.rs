#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
}

#[test]
fn it_should_match_serde_output() {
    let data = r#"{ "age": 43 }"#;

    let output = Person::json_from_str(data).unwrap();
    let serde_output: Person = serde_json::from_str(data).unwrap();
    assert_eq!(output, serde_output);
}
