use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct Person {
    age: u8,
}

static DATA: &str = r#"{ "age": 43 }"#;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rigid", |b| {
        b.iter(|| {
            Person::from_json(black_box(DATA)).unwrap();
        })
    });
    c.bench_function("serde", |b| {
        b.iter(|| {
            serde_json::from_str::<Person>(black_box(DATA)).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
