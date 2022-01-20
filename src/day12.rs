use std::collections::HashMap;
use std::default::Default;
use std::ops::{Index, IndexMut};
use std::time::Instant;

pub fn solve1(input: &[String]) {
    let start = Instant::now();
    let mut edges = CaveMap::new();
    let mut parser = CaveParser::new();
    for (x, y) in input.iter().map(|line| line.split_once("-").unwrap()) {
        let x = parser.parse(x);
        let y = parser.parse(y);
        edges[x].get_or_insert_with(|| Vec::new()).push(y);
        edges[y].get_or_insert_with(|| Vec::new()).push(x);
    }
    let parsing = Instant::now();
    let mut count = 0;
    let mut stack = vec![(Cave::Start, 0)];
    while let Some((head, seen)) = stack.pop() {
        for &neighbor in edges[head].as_ref().unwrap() {
            if neighbor == Cave::Start {
                continue;
            }
            if neighbor == Cave::End {
                count += 1;
                continue;
            }
            let mut new_seen = seen;
            if let Some(neighbor_onehot) = neighbor.small_onehot() {
                if (seen & neighbor_onehot) > 0 {
                    continue;
                }
                new_seen |= neighbor_onehot;
            }
            stack.push((neighbor, new_seen));
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

pub struct CaveParser<'a> {
    smalls: Vec<&'a str>,
    larges: Vec<&'a str>,
}

impl<'a> CaveParser<'a> {
    pub fn new() -> Self {
        CaveParser {
            smalls: Vec::new(),
            larges: Vec::new(),
        }
    }
    pub fn parse(&mut self, k: &'a str) -> Cave {
        if k == "start" {
            return Cave::Start;
        }
        if k == "end" {
            return Cave::End;
        }
        let is_small = k.chars().next().unwrap().is_lowercase();
        if is_small {
            match self.smalls.iter().position(|x| *x == k) {
                None => {
                    self.smalls.push(k);
                    assert!(self.smalls.len() <= 16);
                    Cave::Small(self.smalls.len() as u8 - 1)
                }
                Some(i) => Cave::Small(i as u8),
            }
        } else {
            match self.larges.iter().position(|x| *x == k) {
                None => {
                    self.larges.push(k);
                    assert!(self.larges.len() <= 16);
                    Cave::Large(self.larges.len() as u8 - 1)
                }
                Some(i) => Cave::Large(i as u8),
            }
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
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

pub fn solve2_inner(edges: &CaveMap<Vec<Cave>>) -> usize {
    let mut small_loops: HashMap<u16, usize> = HashMap::new();
    for i in 0..16 {
        //for (k, onehot) in edges.keys().filter_map(|k| Some((*k, k.small_onehot()?))) {
        let k = Cave::Small(i);
        if edges[k].is_none() {
            continue;
        }
        let onehot = 1 << i;
        let mut stack = vec![(k, onehot)];
        while let Some((head, seen)) = stack.pop() {
            for &neighbor in edges[head].as_ref().unwrap() {
                if neighbor == Cave::Start {
                    continue;
                }
                if neighbor == Cave::End {
                    continue;
                }
                if neighbor == k {
                    *small_loops.entry(seen).or_insert(0) += 1;
                    continue;
                }
                let mut new_seen = seen;
                if let Some(neighbor_onehot) = neighbor.small_onehot() {
                    if (seen & neighbor_onehot) > 0 || neighbor < k {
                        continue;
                    }
                    new_seen |= neighbor_onehot;
                }
                stack.push((neighbor, new_seen));
            }
        }
    }
    let mut count = 0;
    let mut stack = vec![(Cave::Start, 0)];
    while let Some((head, seen)) = stack.pop() {
        for &neighbor in edges[head].as_ref().unwrap() {
            if neighbor == Cave::Start {
                continue;
            }
            if neighbor == Cave::End {
                count += 1;
                for (small_loop, loop_count) in small_loops.iter() {
                    if (small_loop & seen).count_ones() == 1 {
                        count += loop_count;
                    }
                }
                continue;
            }
            let mut new_seen = seen;
            if let Some(neighbor_onehot) = neighbor.small_onehot() {
                if (seen & neighbor_onehot) > 0 {
                    continue;
                }
                new_seen |= neighbor_onehot;
            }
            stack.push((neighbor, new_seen));
        }
    }
    count
}
