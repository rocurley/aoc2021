use aoc2021::{parse, solve};
use cpuprofiler::PROFILER;
use criterion::profiler::Profiler;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;
use std::sync::MutexGuard;

const input: [&'static str; 23] = [
    "yw-MN", "wn-XB", "DG-dc", "MN-wn", "yw-DG", "start-dc", "start-ah", "MN-start", "fi-yw",
    "XB-fi", "wn-ah", "MN-ah", "MN-dc", "end-yw", "fi-end", "th-fi", "end-XB", "dc-XB", "yw-XN",
    "wn-yw", "dc-ah", "MN-fi", "wn-DG",
];

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve2", |b| {
        b.iter(|| {
            let edges = parse(black_box(&input));
            solve(&edges)
        })
    });
}

criterion_group! {
name = benches;
config = profiled();
targets = criterion_benchmark
}
criterion_main!(benches);

fn profiled() -> Criterion {
    Criterion::default().with_profiler(CPUProfiler { profiler: None })
}

struct CPUProfiler<'a> {
    profiler: Option<MutexGuard<'a, cpuprofiler::Profiler>>,
}

impl<'a> Profiler for CPUProfiler<'a> {
    fn start_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        let mut profiler = PROFILER.lock().unwrap();
        profiler
            .start(format!("profiling/{}", benchmark_id))
            .unwrap();
        self.profiler = Some(profiler);
    }
    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        let mut p = self.profiler.take().unwrap();
        p.stop().unwrap();
    }
}
