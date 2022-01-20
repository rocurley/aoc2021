use std::collections::HashMap;
use std::default::Default;
use std::ops::{Index, IndexMut};
use std::time::Instant;

// TODO: Eliminate large caves entirely! You can do this by storing a weight between any two small
// caves. This weight is the number of paths between them using only large caves. This should make
// it possible to slim things down quite a bit.
pub fn solve1(input: &[String]) {
    let start = Instant::now();
    let mut big_edges = HashMap::new();
    let mut parser = CaveParser::new();
    let mut edges_raw = HashMap::new();
    for (x, y) in input.iter().map(|line| line.split_once("-").unwrap()) {
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
            for &y in small_caves[i + 1..].iter() {
                let k = if x < y { (x, y) } else { (y, x) };
                *edges_raw.entry(k).or_insert(0) += 1;
            }
        }
    }
    let mut edges = CaveMap::new();
    for ((x, y), c) in edges_raw.into_iter() {
        edges[x].get_or_insert(Vec::new()).push((y, c));
        edges[y].get_or_insert(Vec::new()).push((x, c));
    }
    let parsing = Instant::now();
    let mut count = 0;
    let mut stack = vec![Path {
        head: Cave::Start,
        seen: 0,
        weight: 1,
    }];
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
            if neighbor == Cave::Start {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == Cave::End {
                count += weight;
                continue;
            }
            let mut new_seen = path.seen;
            if let Some(neighbor_onehot) = neighbor.small_onehot() {
                if (path.seen & neighbor_onehot) > 0 {
                    continue;
                }
                new_seen |= neighbor_onehot;
            }
            stack.push(Path {
                head: neighbor,
                seen: new_seen,
                weight,
            });
        }
    }
    let part1_solve = Instant::now();
    dbg!(count);
    let part1_print = Instant::now();

    let count = solve2_inner(&edges);
    let part2_solve = Instant::now();
    dbg!(count);
    let part2_print = Instant::now();
    println!("Parsing: {:?}", parsing - start);
    println!("Part 1 solve: {:?}", part1_solve - parsing);
    println!("Part 1 print: {:?}", part1_print - part1_solve);
    println!("Part 2 solve: {:?}", part2_solve - part1_print);
    println!("Part 2 print: {:?}", part2_print - part2_solve);
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
        if k == "start" {
            return Some(Cave::Start);
        }
        if k == "end" {
            return Some(Cave::End);
        }
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
    Start,
    End,
    Small(u8),
    Large(u8),
}

impl Cave {
    fn is_small(&self) -> bool {
        match self {
            Cave::Small(_) => true,
            _ => false,
        }
    }
    fn small_onehot(&self) -> Option<u16> {
        match self {
            Cave::Small(x) => Some(1 << x),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct CaveSet {
    start: bool,
    end: bool,
    small: u16,
    large: u16,
}

impl CaveSet {
    fn new() -> Self {
        CaveSet {
            start: false,
            end: false,
            small: 0,
            large: 0,
        }
    }
    fn insert(&mut self, x: Cave) {
        match x {
            Cave::Start => self.start = true,
            Cave::End => self.end = true,
            Cave::Small(i) => self.small |= 1 << i,
            Cave::Large(i) => self.large |= 1 << i,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct CaveMap<T> {
    start: Option<T>,
    end: Option<T>,
    small: [Option<T>; 16],
    large: [Option<T>; 16],
}

impl<T> CaveMap<T> {
    pub fn new() -> Self {
        CaveMap {
            start: None,
            end: None,
            small: Default::default(),
            large: Default::default(),
        }
    }
}

impl<T> Index<Cave> for CaveMap<T> {
    type Output = Option<T>;
    fn index(&self, ix: Cave) -> &Option<T> {
        match ix {
            Cave::Start => &self.start,
            Cave::End => &self.end,
            Cave::Small(i) => &self.small[i as usize],
            Cave::Large(i) => &self.large[i as usize],
        }
    }
}

impl<T> IndexMut<Cave> for CaveMap<T> {
    fn index_mut(&mut self, ix: Cave) -> &mut Option<T> {
        match ix {
            Cave::Start => &mut self.start,
            Cave::End => &mut self.end,
            Cave::Small(i) => &mut self.small[i as usize],
            Cave::Large(i) => &mut self.large[i as usize],
        }
    }
}

/*
fn iter_caves() -> impl Iterator<Item=Cave> {
}
*/

pub fn solve2_inner(edges: &CaveMap<Vec<(Cave, usize)>>) -> usize {
    let mut small_loops: HashMap<u16, usize> = HashMap::new();
    for i in 0..16 {
        let k = Cave::Small(i);
        if edges[k].is_none() {
            continue;
        }
        let onehot = 1 << i;
        let mut stack = vec![Path {
            head: k,
            seen: onehot,
            weight: 1,
        }];
        while let Some(path) = stack.pop() {
            for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
                if neighbor == Cave::Start {
                    continue;
                }
                if neighbor == Cave::End {
                    continue;
                }
                let weight = path.weight * neighbor_weight;
                if neighbor == k {
                    *small_loops.entry(path.seen).or_insert(0) += weight;
                    continue;
                }
                let mut new_seen = path.seen;
                if let Some(neighbor_onehot) = neighbor.small_onehot() {
                    if (path.seen & neighbor_onehot) > 0 || neighbor < k {
                        continue;
                    }
                    new_seen |= neighbor_onehot;
                }
                stack.push(Path {
                    head: neighbor,
                    seen: new_seen,
                    weight,
                });
            }
        }
    }
    dbg!(&small_loops);
    let mut one_count = 0;
    let mut count = 0;
    let mut stack = vec![Path {
        head: Cave::Start,
        seen: 0,
        weight: 1,
    }];
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
            if neighbor == Cave::Start {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == Cave::End {
                count += weight;
                one_count += weight;
                for (small_loop, loop_count) in small_loops.iter() {
                    if (small_loop & path.seen).count_ones() == 1 {
                        count += weight * loop_count;
                    }
                }
                continue;
            }
            let mut new_seen = path.seen;
            if let Some(neighbor_onehot) = neighbor.small_onehot() {
                if (path.seen & neighbor_onehot) > 0 {
                    continue;
                }
                new_seen |= neighbor_onehot;
            }
            stack.push(Path {
                head: neighbor,
                seen: new_seen,
                weight,
            });
        }
    }
    dbg!(one_count);
    count
}
