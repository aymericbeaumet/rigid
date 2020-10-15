use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, serde::Deserialize, rigid::JSONParser)]
struct Person {
    age: u8,
}

static data: &str = r#"{ "age": 43 }"#;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rigid", |b| {
        b.iter(|| {
            Person::json_from_str(black_box(data)).unwrap();
        })
    });
    c.bench_function("serde", |b| {
        b.iter(|| {
            serde_json::from_str::<Person>(black_box(data)).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
