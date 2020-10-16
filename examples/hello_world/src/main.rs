#[derive(Debug, rigid::JSONParser)]
struct Person {
    age: u8,
}

fn main() {
    let input = r#" { "age": 42 } "#;
    let person = Person::from_json_str(input).expect("Unable to parse JSON");
    println!("{:?}", person);
}
