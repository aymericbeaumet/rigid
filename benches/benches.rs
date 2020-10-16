use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct Person {
    age: u8,
}

static DATA: &str = r#"{ "age": 43 }"#;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse JSON");

    group.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(Person::from_json(black_box(DATA)).unwrap());
        })
    });

    group.bench_function("json::parse", |b| {
        b.iter(|| {
            black_box(json::parse(black_box(DATA)).unwrap());
        })
    });

    group.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<Person>(black_box(DATA)).unwrap());
        })
    });

    group.bench_function("simd_json::to_borrowed_value", |b| {
        let mut data = DATA.as_bytes().to_vec();
        b.iter(|| {
            black_box(simd_json::to_borrowed_value(black_box(&mut data)).unwrap());
        })
    });

    group.bench_function("simd_json::to_owned_value", |b| {
        let mut data = DATA.as_bytes().to_vec();
        b.iter(|| {
            black_box(simd_json::to_owned_value(black_box(&mut data)).unwrap());
        })
    });

    group.bench_function("simd_json::serde::from_slice", |b| {
        let mut data = DATA.as_bytes().to_vec();
        b.iter(|| {
            black_box(simd_json::serde::from_slice::<Person>(black_box(&mut data)).unwrap());
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
