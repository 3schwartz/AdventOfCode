use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day9;

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All");
    group.sample_size(10);
    group.measurement_time(Duration::new(30, 0));

    let input = "464 players; last marble is worth 71730 points";
    
    // 1
    let game_1 = day9::Game::new(input, 1);

    group.bench_function("Hashmap - 1", |b| b.iter(|| {
        black_box(game_1.find_highest_score())
    }));
    group.bench_function("VecDeque - 1", |b| b.iter(|| {
        black_box(game_1.find_highest_score_deque())
    }));
    group.bench_function("Vec - 1", |b| b.iter(|| {
        black_box(game_1.find_highest_score_vec())
    }));

    // 5
    let game_5 = day9::Game::new(input, 5);

    group.bench_function("Hashmap - 5", |b| b.iter(|| {
        black_box(game_5.find_highest_score())
    }));
    group.bench_function("VecDeque - 5", |b| b.iter(|| {
        black_box(game_5.find_highest_score_deque())
    }));
    group.bench_function("Vec - 5", |b| b.iter(|| {
        black_box(game_5.find_highest_score_vec())
    }));

    // 10
    // let game_10 = day9::Game::new(input, 10);

    // group.bench_function("Hashmap - 10", |b| b.iter(|| {
    //     black_box(game_10.find_highest_score())
    // }));
    // group.bench_function("VecDeque - 10", |b| b.iter(|| {
    //     black_box(game_10.find_highest_score_deque())
    // }));

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
