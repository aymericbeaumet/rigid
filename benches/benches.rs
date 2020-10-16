use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct Person {
    height: u8,
    birth_year: u16,
    name: String,
    alive: bool,
}

fn criterion_benchmark(c: &mut Criterion) {
    let data: &str =
        r#" { "height": 187 , "birth_year": 1992 , "name": "aymeric" , "alive" : true } "#;

    let mut speed = c.benchmark_group("speed");

    //speed.bench_function("baseline", |b| {
    //b.iter(|| {
    //black_box(black_box(data));
    //})
    //});

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(Person::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<Person>(black_box(data)).unwrap());
        })
    });

    //speed.bench_function("json::parse", |b| {
    //b.iter(|| {
    //black_box(json::parse(black_box(data)).unwrap());
    //})
    //});

    //speed.bench_function("simd_json::serde::from_slice", |b| {
    //let mut data_mut = data.as_bytes().to_vec();
    //b.iter(|| {
    //black_box(simd_json::serde::from_slice::<Person>(black_box(&mut data_mut)).unwrap());
    //})
    //});

    //speed.bench_function("simd_json::to_borrowed_value", |b| {
    //let mut data_mut = data.as_bytes().to_vec();
    //b.iter(|| {
    //black_box(simd_json::to_borrowed_value(black_box(&mut data_mut)).unwrap());
    //})
    //});

    //speed.bench_function("simd_json::to_owned_value", |b| {
    //let mut data_mut = data.as_bytes().to_vec();
    //b.iter(|| {
    //black_box(simd_json::to_owned_value(black_box(&mut data_mut)).unwrap());
    //})
    //});

    speed.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
