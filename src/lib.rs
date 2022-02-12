use smallvec::SmallVec;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd, Debug)]
pub struct Cave(u8);
impl Cave {
    fn onehot(&self) -> Bitvector {
        1 << self.0
    }
}

const START: Cave = Cave(0x0fe);
const END: Cave = Cave(0x0ff);
const MAX_SMALL: usize = std::mem::size_of::<Bitvector>() * 8;
const SMALLVEC_LEN: usize = MAX_SMALL + 2;

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

type Bitvector = u16;
type Edges = CaveMap<SmallVec<[(Cave, u32); SMALLVEC_LEN]>>;
type PathWeights = Vec<u32>;

pub struct Path {
    head: Cave,
    seen: Bitvector,
    weight: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct CaveMap<T> {
    start: T,
    end: T,
    small: SmallVec<[T; MAX_SMALL]>,
}

impl<T: Clone> CaveMap<T> {
    fn new_cloned(init: T, n_caves: u8) -> Self {
        let small = vec![init.clone(); n_caves as usize].into();
        CaveMap {
            start: init.clone(),
            end: init,
            small,
        }
    }
}

impl<T> CaveMap<T> {
    fn count(&self) -> u8 {
        self.small.len() as u8
    }
}

impl<T> Index<Cave> for CaveMap<T> {
    type Output = T;
    fn index(&self, ix: Cave) -> &T {
        match ix {
            x if x == START => &self.start,
            x if x == END => &self.end,
            Cave(i) => &self.small[i as usize],
        }
    }
}

impl<T> IndexMut<Cave> for CaveMap<T> {
    fn index_mut(&mut self, ix: Cave) -> &mut T {
        match ix {
            x if x == START => &mut self.start,
            x if x == END => &mut self.end,
            Cave(i) => &mut self.small[i as usize],
        }
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
    let n_small_caves = parser.smalls.len() as u8;
    let mut edges = CaveMap::new_cloned(SmallVec::new(), n_small_caves);
    for ((x, y), c) in edges_raw.into_iter() {
        edges[x].push((y, c));
        if x != y {
            edges[y].push((x, c));
        }
    }
    edges
}

pub fn find_small_loops(edges: &Edges) -> PathWeights {
    let mut small_loops: PathWeights = vec![0; 1 << edges.count()];
    for i in 0..edges.count() {
        let k = Cave(i);
        let mut stack = vec![Path {
            head: k,
            seen: k.onehot(),
            weight: 1,
        }];
        while let Some(path) = stack.pop() {
            for &(neighbor, neighbor_weight) in &edges[path.head] {
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
                    small_loops[path.seen as usize] += weight;
                    continue;
                }
                let neighbor_onehot = neighbor.onehot();
                if path.seen & neighbor_onehot > 0 {
                    continue;
                }
                stack.push(Path {
                    head: neighbor,
                    seen: path.seen | neighbor_onehot,
                    weight,
                });
            }
        }
    }
    small_loops
}

pub fn find_paths<'a>(edges: &Edges) -> PathWeights {
    let mut stack = vec![Path {
        head: START,
        seen: 0,
        weight: 1,
    }];
    let mut paths: PathWeights = vec![0; 1 << edges.count()];
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in &edges[path.head] {
            if neighbor == START {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == END {
                paths[path.seen as usize] += weight;
                continue;
            }
            let neighbor_onehot = neighbor.onehot();
            if path.seen & neighbor_onehot > 0 {
                continue;
            }
            stack.push(Path {
                head: neighbor,
                seen: path.seen | neighbor_onehot,
                weight,
            });
        }
    }
    paths
}

pub fn join(small_loops: &PathWeights, paths: &PathWeights) -> (u32, u32) {
    let mut count = 0;
    let mut one_count = 0;
    for (path, weight) in paths.iter().enumerate() {
        count += weight;
        one_count += weight;
        let mut intersecting_loop_count = 0;
        for (small_loop, loop_count) in small_loops.iter().enumerate() {
            if (small_loop & path).count_ones() == 1 {
                intersecting_loop_count += loop_count;
            }
        }
        count += intersecting_loop_count * weight;
    }
    (one_count, count)
}

pub fn solve(edges: &Edges) -> (u32, u32) {
    let small_loops = find_small_loops(edges);
    let paths = find_paths(edges);
    join(&small_loops, &paths)
}
