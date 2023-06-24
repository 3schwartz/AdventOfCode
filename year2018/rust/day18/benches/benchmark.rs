use std::{time::Duration, fs};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day18;

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All");
    group.sample_size(10);
    group.measurement_time(Duration::new(30, 0));

    let input = fs::read_to_string("../../data/day18_data.txt").unwrap();
    let map = day18::LumberCollection::from(&input).unwrap();
    let iterations = 200;
    
    group.bench_function("Many collections", |b| {
        b.iter(|| black_box(map.find_resource_after_iterations_using_sumple(iterations)))
    });
    group.bench_function("Simple", |b| {
        b.iter(|| black_box(map.find_resource_after_iterations(iterations,0)))
    });
    
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
