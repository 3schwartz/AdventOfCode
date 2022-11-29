use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::simple;
use day5::value;

pub fn all_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../../data/day5_data.txt")
                .expect("couldn't open file");

    let mut group = c.benchmark_group("All");
    group.sample_size(10);
    group.measurement_time(Duration::new(120, 0));

    group.bench_function("Simple - All", |b| b.iter(|| {
        let mut polymer = simple::Polymer::new(&input);
        black_box(polymer.find_polymer_length());
    }));
    group.bench_function("Value - All", |b| b.iter(|| black_box(value::get_polymer_length(&input))));

    group.finish();
}

/// All
pub fn value_all_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../../data/day5_data.txt")
                .expect("couldn't open file");
    c.bench_function("Value - All", |b| b.iter(|| black_box(value::get_polymer_length(&input))));
}

criterion_group!(benches, 
    // simple_small_benchmark, value_small_benchmark,
    all_benchmark);
criterion_main!(benches);