use aoc2021::*;
use argh::FromArgs;

#[cfg(feature = "profiler")]
use cpuprofiler::PROFILER;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

#[derive(FromArgs)]
/// Solve problems!
struct Args {
    /// which problem to solve
    #[argh(option, short = 'p')]
    problem: String,

    /// path for the input
    #[argh(positional)]
    input_path: String,

    /// enable profiling
    #[argh(switch)]
    profile: bool,
}

fn main() {
    let args: Args = argh::from_env();
    let file = fs::File::open(args.input_path).expect("couldn't read input");
    #[cfg(feature = "profiler")]
    let profiler = if args.profile {
        let mut profiler = PROFILER.lock().unwrap();
        profiler
            .start(format!("profiling/{}", args.problem.as_str()))
            .unwrap();
        let now = Instant::now();
        Some((profiler, now))
    } else {
        None
    };
    let input = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("coudln't read line");
    match args.problem.as_str() {
        "day1" => {
            day1::solve1(&input);
            day1::solve2(&input);
        }
        _ => panic!("Unexpected problem name"),
    }
    #[cfg(feature = "profiler")]
    if let Some((mut p, start)) = profiler {
        let t = start.elapsed();
        p.stop().unwrap();
        println!("{:?}", t);
    }
}
