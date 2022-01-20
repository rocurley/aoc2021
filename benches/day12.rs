use aoc2021::day12::{solve2_inner, CaveMap, CaveParser};
use criterion::profiler::Profiler;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::protos::Message;
use pprof::ProfilerGuard;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const input: [&'static str; 23] = [
    "yw-MN", "wn-XB", "DG-dc", "MN-wn", "yw-DG", "start-dc", "start-ah", "MN-start", "fi-yw",
    "XB-fi", "wn-ah", "MN-ah", "MN-dc", "end-yw", "fi-end", "th-fi", "end-XB", "dc-XB", "yw-XN",
    "wn-yw", "dc-ah", "MN-fi", "wn-DG",
];

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve2", |b| {
        b.iter(|| {
            let mut edges = CaveMap::new();
            let mut parser = CaveParser::new();
            for (x, y) in input.iter().map(|line| line.split_once("-").unwrap()) {
                let x = parser.parse(x);
                let y = parser.parse(y);
                edges[x].get_or_insert_with(|| Vec::new()).push(y);
                edges[y].get_or_insert_with(|| Vec::new()).push(x);
            }
            solve2_inner(&edges)
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
