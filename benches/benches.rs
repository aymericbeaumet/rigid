use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
}

static data: &str = r#"{ "age": 43 }"#;

fn rigid(n: u64) -> Person {
    Person::json_from_str(data).unwrap()
}

fn serde(n: u64) -> Person {
    serde_json::from_str(data).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rigid", |b| b.iter(|| rigid(black_box(20))));
    c.bench_function("serde", |b| b.iter(|| serde(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
