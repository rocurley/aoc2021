use aoc2021::day6::{step_fish, step_fish_fast_1, step_fish_fast_2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("step_fish", |b| {
        b.iter(|| {
            let mut fish = [0, 79, 42, 52, 56, 71, 0, 0, 0];
            for _ in 0..256 {
                step_fish(black_box(&mut fish))
            }
        })
    });
    c.bench_function("step_fish_fast_1", |b| {
        b.iter(|| {
            let mut fish = [0, 79, 42, 52, 56, 71, 0, 0, 0];
            step_fish_fast_1(256, &mut fish);
        })
    });
    c.bench_function("step_fish_fast_2", |b| {
        b.iter(|| {
            let mut fish = [0, 79, 42, 52, 56, 71, 0, 0, 0];
            step_fish_fast_2(256, &mut fish);
        })
    });
    c.bench_function("step_fish_fast_3", |b| {
        b.iter(|| {
            let mut fish = [0, 79, 42, 52, 56, 71, 0, 0, 0];
            step_fish_fast_2(256, &mut fish);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
