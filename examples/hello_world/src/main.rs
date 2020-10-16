#[derive(Debug, rigid::FromJSON)]
struct Person {
    age: u8,
}

fn main() {
    let input = r#" { "age": 42 } "#;
    let person = Person::from_json(input).expect("Unable to parse JSON");
    println!("{:?}", person);
}
