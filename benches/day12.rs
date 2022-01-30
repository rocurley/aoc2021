use aoc2021::day12::{find_paths, find_paths_ref, parse, solve_inner};
use cpuprofiler::PROFILER;
use criterion::profiler::Profiler;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::protos::Message;
use pprof::ProfilerGuard;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::MutexGuard;

const input: [&'static str; 23] = [
    "yw-MN", "wn-XB", "DG-dc", "MN-wn", "yw-DG", "start-dc", "start-ah", "MN-start", "fi-yw",
    "XB-fi", "wn-ah", "MN-ah", "MN-dc", "end-yw", "fi-end", "th-fi", "end-XB", "dc-XB", "yw-XN",
    "wn-yw", "dc-ah", "MN-fi", "wn-DG",
];

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("combined", |b| {
        b.iter(|| {
            let (edges, count) = parse(black_box(&input));
            solve_inner(&edges, count)
        })
    });
    c.bench_function("parse", |b| b.iter(|| parse(black_box(&input))));
    c.bench_function("solve", |b| {
        let (edges, count) = parse(&input);
        b.iter(|| solve_inner(black_box(&edges), black_box(count)))
    });
    c.bench_function("find_paths", |b| {
        let (edges, count) = parse(&input);
        b.iter(|| find_paths(black_box(&edges), black_box(count)))
    });
    c.bench_function("find_paths_ref", |b| {
        let (edges, count) = parse(&input);
        b.iter(|| find_paths_ref(black_box(&edges), black_box(count)))
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
    profiler: Option<pprof::ProfilerGuard<'a>>,
}

impl<'a> Profiler for CPUProfiler<'a> {
    fn start_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        self.profiler = Some(ProfilerGuard::new(100000).unwrap());
    }
    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        let p = self.profiler.take().unwrap();
        let report = p.report().build().unwrap();
        let mut file = File::create(format!("profiling/{}", benchmark_id)).unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();
        let file = File::create(format!("profiling/{}.svg", benchmark_id)).unwrap();
        report.flamegraph(file).unwrap();
    }
}

struct CPUProfiler2<'a> {
    profiler: Option<MutexGuard<'a, cpuprofiler::Profiler>>,
}

impl<'a> Profiler for CPUProfiler2<'a> {
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
