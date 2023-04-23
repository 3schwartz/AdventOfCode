use std::{time::Duration, collections::HashMap};

use day7::{self, find_signal};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../../data/day7_data.txt")
                .expect("couldn't open file");

    let mut group = c.benchmark_group("all");
    group.sample_size(10);
    group.measurement_time(Duration::new(20,0));

    group.bench_function("lines", |b| b.iter(|| {
        let lines = input.lines();

        let mut signals: HashMap<&str, u16> = HashMap::new();

        black_box(find_signal(&mut signals, &lines, false).unwrap());
    }));

    group.bench_function("vec", |b| b.iter(|| {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();

        let mut signals: HashMap<&str, u16> = HashMap::new();

        black_box(find_signal(&mut signals, &lines, false).unwrap());
    }));

    let lines = input.lines();
    group.bench_function("lines - only function", |b| b.iter(|| {
        let mut signals: HashMap<&str, u16> = HashMap::new();

        black_box(find_signal(&mut signals, &lines, false).unwrap());
    }));

    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    group.bench_function("vec - only function", |b| b.iter(|| {
        let mut signals: HashMap<&str, u16> = HashMap::new();

        black_box(find_signal(&mut signals, &lines, false).unwrap());
    }));

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);