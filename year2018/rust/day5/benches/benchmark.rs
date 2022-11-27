use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::simple;

pub fn simple_benchmark(c: &mut Criterion) {
    let input = "dabAcCaCBAcCcaDA";
    c.bench_function("Simple", |b| b.iter(|| black_box(simple::get_polymer_length(input))));
}

criterion_group!(benches, simple_benchmark);
criterion_main!(benches);