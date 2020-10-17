use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct StringOnly(String);

fn criterion_struct_one_field_tuple(c: &mut Criterion) {
    let data: &str = r#""foobar""#;

    let mut speed = c.benchmark_group("one_field_tuple");

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(StringOnly::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<StringOnly>(black_box(data)).unwrap());
        })
    });

    speed.finish();
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct StructEmpty {}

fn criterion_struct_empty_many_spaces(c: &mut Criterion) {
    let data: &str = r#"          {          }          "#;

    let mut speed = c.benchmark_group("empty_many_spaces");

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(StructEmpty::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<StructEmpty>(black_box(data)).unwrap());
        })
    });

    speed.finish();
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct StructBool {
    height: bool,
}

fn criterion_struct_bool(c: &mut Criterion) {
    let data: &str = r#" { "height" : true } "#;

    let mut speed = c.benchmark_group("bool");

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(StructBool::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<StructBool>(black_box(data)).unwrap());
        })
    });

    speed.finish();
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct StructU8 {
    height: u8,
}

fn criterion_struct_u8(c: &mut Criterion) {
    let data: &str = r#" { "height" : 190 } "#;

    let mut speed = c.benchmark_group("u8");

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(StructU8::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<StructU8>(black_box(data)).unwrap());
        })
    });

    speed.finish();
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct StructU16 {
    height: u16,
}

fn criterion_struct_u16(c: &mut Criterion) {
    let data: &str = r#" { "height" : 190 } "#;

    let mut speed = c.benchmark_group("u16");

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(StructU16::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<StructU16>(black_box(data)).unwrap());
        })
    });

    speed.finish();
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct StructString {
    height: String,
}

fn criterion_struct_string(c: &mut Criterion) {
    let data: &str = r#" { "height" : "190" } "#;

    let mut speed = c.benchmark_group("string");

    speed.bench_function("rigid::from_json", |b| {
        b.iter(|| {
            black_box(StructString::from_json(black_box(data)).unwrap());
        })
    });

    speed.bench_function("serde_json::from_str", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<StructString>(black_box(data)).unwrap());
        })
    });

    speed.finish();
}

#[derive(Debug, PartialEq, serde::Deserialize, rigid::FromJSON)]
struct Person {
    height: u8,
    birth_year: u16,
    name: String,
    alive: bool,
}

fn criterion_person(c: &mut Criterion) {
    let data: &str =
        r#" { "height": 187 , "birth_year": 1992 , "name": "aymeric" , "alive" : true } "#;

    let mut speed = c.benchmark_group("person");

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

    speed.finish();
}

criterion_group!(
    benches,
    criterion_struct_one_field_tuple,
    criterion_struct_empty_many_spaces,
    criterion_struct_bool,
    criterion_struct_u8,
    criterion_struct_u16,
    criterion_struct_string,
    criterion_person
);
criterion_main!(benches);
