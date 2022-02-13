use aoc2021::{find_paths_and_join, find_small_loops, parse, solve};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const input: [&'static str; 23] = [
    "yw-MN", "wn-XB", "DG-dc", "MN-wn", "yw-DG", "start-dc", "start-ah", "MN-start", "fi-yw",
    "XB-fi", "wn-ah", "MN-ah", "MN-dc", "end-yw", "fi-end", "th-fi", "end-XB", "dc-XB", "yw-XN",
    "wn-yw", "dc-ah", "MN-fi", "wn-DG",
];

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("combined", |b| {
        b.iter(|| {
            let edges = parse(black_box(&input));
            solve(&edges)
        })
    });
    c.bench_function("parse", |b| b.iter(|| parse(black_box(&input))));
    c.bench_function("solve", |b| {
        let edges = parse(&input);
        b.iter(|| solve(black_box(&edges)))
    });
    c.bench_function("find_paths_and_join", |b| {
        let edges = parse(&input);
        let small_loops = find_small_loops(&edges);
        b.iter(|| find_paths_and_join(black_box(&edges), black_box(&small_loops)))
    });
    c.bench_function("find_small_loops", |b| {
        let edges = parse(&input);
        b.iter(|| find_small_loops(black_box(&edges)))
    });
}

criterion_group! {
name = benches;
config = profiled();
targets = criterion_benchmark
}
criterion_main!(benches);

fn profiled() -> Criterion {
    Criterion::default()
}
