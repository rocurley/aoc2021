use aoc2021::{parse, solve};

use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("input").expect("couldn't read input");
    let input = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("coudln't read line");
    let edges = parse(&input);
    let (part1, part2) = solve(&edges);
    dbg!(part1, part2);
}
