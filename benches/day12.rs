use aoc2021::day12::{find_loops, find_paths, find_paths_ref, join, parse, solve_inner};
use cpuprofiler::PROFILER;
use criterion::profiler::Profiler;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::protos::Message;
use pprof::ProfilerGuard;
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
            let edges = parse(black_box(&input));
            solve_inner(&edges)
        })
    });
    c.bench_function("parse", |b| b.iter(|| parse(black_box(&input))));
    c.bench_function("solve", |b| {
        let edges = parse(&input);
        b.iter(|| solve_inner(black_box(&edges)))
    });
    c.bench_function("join", |b| {
        let edges = parse(&input);
        let small_loops = find_loops(&edges);
        let paths = find_paths(&edges);
        b.iter(|| join(black_box(&paths), black_box(&small_loops)))
    });
    c.bench_function("find_paths", |b| {
        let edges = parse(&input);
        b.iter(|| find_paths(black_box(&edges)))
    });
    c.bench_function("find_paths_ref", |b| {
        let edges = parse(&input);
        b.iter(|| find_paths_ref(black_box(&edges)))
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
