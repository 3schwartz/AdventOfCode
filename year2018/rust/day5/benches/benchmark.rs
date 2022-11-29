use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::simple;
use day5::value;

pub fn simple_benchmark(c: &mut Criterion) {
    let input = "dabAcCaCBAcCcaDA";
    c.bench_function("Simple", |b| b.iter(|| {
        let mut polymer = simple::Polymer::new(&input);
        black_box(polymer.find_polymer_length());
    }));
}

pub fn value_benchmark(c: &mut Criterion) {
    let input = "dabAcCaCBAcCcaDA";
    c.bench_function("Value", |b| b.iter(|| black_box(value::get_polymer_length(input))));
}

criterion_group!(benches, simple_benchmark, value_benchmark);
criterion_main!(benches);