use std::collections::HashMap;
use std::default::Default;
use std::ops::{Index, IndexMut};
use std::time::Instant;

pub fn solve1(input: &[String]) {
    let start_t = Instant::now();
    let (start, end, edges) = parse(input);
    let parsing = Instant::now();
    let (sol1, sol2) = solve_inner(start, end, &edges);
    let solve = Instant::now();
    println!("Part 1: {}", sol1);
    println!("Part 2: {}", sol2);
    let print = Instant::now();
    println!("Parsing: {:?}", parsing - start_t);
    println!("Solve: {:?}", solve - parsing);
    println!("Print: {:?}", print - solve);
}

pub struct Path {
    head: Cave,
    seen: u16,
    weight: usize,
}

pub struct CaveParser<'a> {
    smalls: Vec<&'a str>,
}

impl<'a> CaveParser<'a> {
    pub fn new() -> Self {
        CaveParser { smalls: Vec::new() }
    }
    pub fn parse(&mut self, k: &'a str) -> Option<Cave> {
        let is_small = k.chars().next().unwrap().is_lowercase();
        if !is_small {
            return None;
        }
        match self.smalls.iter().position(|x| *x == k) {
            None => {
                self.smalls.push(k);
                assert!(self.smalls.len() <= 16);
                Some(Cave::Small(self.smalls.len() as u8 - 1))
            }
            Some(i) => Some(Cave::Small(i as u8)),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd, Debug)]
pub enum Cave {
    Small(u8),
}

impl Cave {
    fn small_onehot(&self) -> u16 {
        match self {
            Cave::Small(x) => 1 << x,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct CaveMap<T> {
    small: [Option<T>; 16],
}

impl<T> CaveMap<T> {
    pub fn new() -> Self {
        CaveMap {
            small: Default::default(),
        }
    }
}

impl<T> Index<Cave> for CaveMap<T> {
    type Output = Option<T>;
    fn index(&self, ix: Cave) -> &Option<T> {
        match ix {
            Cave::Small(i) => &self.small[i as usize],
        }
    }
}

impl<T> IndexMut<Cave> for CaveMap<T> {
    fn index_mut(&mut self, ix: Cave) -> &mut Option<T> {
        match ix {
            Cave::Small(i) => &mut self.small[i as usize],
        }
    }
}

pub fn parse<S: AsRef<str>>(input: &[S]) -> (Cave, Cave, CaveMap<Vec<(Cave, usize)>>) {
    let mut big_edges = HashMap::new();
    let mut parser = CaveParser::new();
    let mut edges_raw = HashMap::new();
    for (x, y) in input
        .iter()
        .map(|line| line.as_ref().split_once("-").unwrap())
    {
        match (parser.parse(x), parser.parse(y)) {
            (None, None) => panic!("Cannot connect two big caves"),
            (Some(x), Some(y)) => {
                let k = if x < y { (x, y) } else { (y, x) };
                assert!(edges_raw.insert(k, 1).is_none());
            }
            (None, Some(y)) => big_edges.entry(x).or_insert(Vec::new()).push(y),
            (Some(x), None) => big_edges.entry(y).or_insert(Vec::new()).push(x),
        }
    }
    for small_caves in big_edges.into_values() {
        for (i, &x) in small_caves.iter().enumerate() {
            for &y in small_caves[i..].iter() {
                let k = if x < y { (x, y) } else { (y, x) };
                *edges_raw.entry(k).or_insert(0) += 1;
            }
        }
    }
    let mut edges = CaveMap::new();
    for ((x, y), c) in edges_raw.into_iter() {
        edges[x].get_or_insert(Vec::new()).push((y, c));
        if x != y {
            edges[y].get_or_insert(Vec::new()).push((x, c));
        }
    }
    (
        parser.parse("start").unwrap(),
        parser.parse("end").unwrap(),
        edges,
    )
}

pub fn solve_inner(start: Cave, end: Cave, edges: &CaveMap<Vec<(Cave, usize)>>) -> (usize, usize) {
    let mut small_loops: HashMap<u16, usize> = HashMap::new();
    let mut stack = Vec::new();
    for i in 0..16 {
        let k = Cave::Small(i);
        if edges[k].is_none() {
            continue;
        }
        let onehot = 1 << i;
        stack.push(Path {
            head: k,
            seen: onehot,
            weight: 1,
        });
        while let Some(path) = stack.pop() {
            for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
                if neighbor == start {
                    continue;
                }
                if neighbor == end {
                    continue;
                }
                let weight = path.weight * neighbor_weight;
                if neighbor == k {
                    *small_loops.entry(path.seen).or_insert(0) += weight;
                    continue;
                }
                let mut new_seen = path.seen;
                let neighbor_onehot = neighbor.small_onehot();
                if (path.seen & neighbor_onehot) > 0 || neighbor < k {
                    continue;
                }
                new_seen |= neighbor_onehot;
                stack.push(Path {
                    head: neighbor,
                    seen: new_seen,
                    weight,
                });
            }
        }
    }
    let mut one_count = 0;
    let mut count = 0;
    let mut stack = vec![Path {
        head: start,
        seen: 0,
        weight: 1,
    }];
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
            if neighbor == start {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == end {
                let mut total_loop_count = 0;
                for (small_loop, loop_count) in small_loops.iter() {
                    if (small_loop & path.seen).is_power_of_two() {
                        total_loop_count += loop_count;
                    }
                }
                count += weight * (1 + total_loop_count);
                one_count += weight;
                continue;
            }
            let mut new_seen = path.seen;
            let neighbor_onehot = neighbor.small_onehot();
            if (path.seen & neighbor_onehot) > 0 {
                continue;
            }
            new_seen |= neighbor_onehot;
            stack.push(Path {
                head: neighbor,
                seen: new_seen,
                weight,
            });
        }
    }
    (one_count, count)
}
