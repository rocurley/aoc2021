use aoc2021::solve1;

use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("input").expect("couldn't read input");
    let input = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("coudln't read line");
    solve1(&input);
}
