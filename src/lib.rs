use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd, Debug)]
pub struct Cave(u8);

const START: Cave = Cave(0x0fe);
const END: Cave = Cave(0x0ff);
const MAX_SMALL: usize = START.0 as usize;

pub struct CaveParser<'a> {
    smalls: Vec<&'a str>,
}

impl<'a> CaveParser<'a> {
    pub fn new() -> Self {
        CaveParser { smalls: Vec::new() }
    }
    pub fn parse(&mut self, k: &'a str) -> Option<Cave> {
        if k == "start" {
            return Some(START);
        }
        if k == "end" {
            return Some(END);
        }
        let is_small = k.chars().next().unwrap().is_lowercase();
        if !is_small {
            return None;
        }
        match self.smalls.iter().position(|x| *x == k) {
            None => {
                self.smalls.push(k);
                assert!(self.smalls.len() <= MAX_SMALL);
                Some(Cave(self.smalls.len() as u8 - 1))
            }
            Some(i) => Some(Cave(i as u8)),
        }
    }
}

type Edges = HashMap<Cave, Vec<(Cave, usize)>>;
type PathWeights = HashMap<Vec<Cave>, usize>;

pub struct Path {
    trail: Vec<Cave>,
    weight: usize,
}

impl Path {
    fn head(&self) -> Cave {
        *self.trail.last().unwrap()
    }
}

pub fn parse<S: AsRef<str>>(input: &[S]) -> Edges {
    let mut big_edges = HashMap::new();
    let mut edges_raw = HashMap::new();
    let mut parser = CaveParser::new();
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
    for (_, small_caves) in big_edges {
        for (i, &x) in small_caves.iter().enumerate() {
            for &y in small_caves[i..].iter() {
                let k = if x < y { (x, y) } else { (y, x) };
                *edges_raw.entry(k).or_insert(0) += 1;
            }
        }
    }
    let mut edges = HashMap::new();
    for ((x, y), c) in edges_raw.into_iter() {
        edges.entry(x).or_insert(Vec::new()).push((y, c));
        if x != y {
            edges.entry(y).or_insert(Vec::new()).push((x, c));
        }
    }
    edges
}

pub fn find_small_loops(edges: &Edges) -> PathWeights {
    let mut small_loops: PathWeights = HashMap::new();
    for &k in edges.keys().filter(|&&k| k != START && k != END) {
        let mut stack = vec![Path {
            trail: vec![k],
            weight: 1,
        }];
        while let Some(path) = stack.pop() {
            for &(neighbor, neighbor_weight) in &edges[&path.head()] {
                if neighbor == START {
                    continue;
                }
                if neighbor == END {
                    continue;
                }
                if neighbor < k {
                    continue;
                }
                let weight = path.weight * neighbor_weight;
                if neighbor == k {
                    let mut ix = path.trail.clone();
                    ix.sort();
                    *small_loops.entry(ix).or_insert(0) += weight;
                    continue;
                }
                if path.trail.contains(&neighbor) {
                    continue;
                }
                let mut new_trail = path.trail.clone();
                new_trail.push(neighbor);
                stack.push(Path {
                    trail: new_trail,
                    weight,
                });
            }
        }
    }
    small_loops
}

pub fn find_paths<'a>(edges: &Edges) -> PathWeights {
    let mut stack = vec![Path {
        trail: vec![START],
        weight: 1,
    }];
    let mut paths: PathWeights = HashMap::new();
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in &edges[&path.head()] {
            if neighbor == START {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == END {
                let mut ix = path.trail.clone();
                ix.sort();
                *paths.entry(ix).or_insert(0) += weight;
                continue;
            }
            if path.trail.contains(&neighbor) {
                continue;
            }
            let mut new_trail = path.trail.clone();
            new_trail.push(neighbor);
            stack.push(Path {
                trail: new_trail,
                weight,
            });
        }
    }
    paths
}

pub fn join(small_loops: &PathWeights, paths: &PathWeights) -> (usize, usize) {
    let mut count = 0;
    let mut one_count = 0;
    for (path, weight) in paths {
        count += weight;
        one_count += weight;
        let mut intersecting_loop_count = 0;
        for (small_loop, loop_count) in small_loops.iter() {
            if small_loop.iter().filter(|k| path.contains(k)).count() == 1 {
                intersecting_loop_count += loop_count;
            }
        }
        count += intersecting_loop_count * weight;
    }
    (one_count, count)
}

pub fn solve(edges: &Edges) -> (usize, usize) {
    let small_loops = find_small_loops(edges);
    let paths = find_paths(edges);
    join(&small_loops, &paths)
}
