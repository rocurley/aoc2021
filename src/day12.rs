use smallvec::SmallVec;
use std::collections::HashMap;
use std::default::Default;
use std::ops::{Index, IndexMut};
use std::time::Instant;

pub fn solve1(input: &[String]) {
    let start_t = Instant::now();
    let edges = parse(input);
    let parsing = Instant::now();
    let (loops_dfs, paths_dfs, (sol1, sol2)) = solve_inner(&edges);
    let solve = Instant::now();
    println!("Part 1: {}", sol1);
    println!("Part 2: {}", sol2);
    let print = Instant::now();
    println!("Parsing: {:?}", parsing - start_t);
    println!("Loops DFS: {:?}", loops_dfs - parsing);
    println!("Paths DFS: {:?}", paths_dfs - loops_dfs);
    println!("Join: {:?}", solve - paths_dfs);
    //println!("Solve: {:?}", solve - parsing);
    println!("Print: {:?}", print - solve);
}

type Bitvector = u16;
const MAX_SMALL: usize = std::mem::size_of::<Bitvector>() * 8;
const SMALLVEC_LEN: usize = MAX_SMALL + 2;

pub struct Path {
    head: Cave,
    seen: Bitvector,
    weight: usize,
}

#[derive(PartialEq, Eq, Hash)]
pub struct PathIndex {
    head: Cave,
    seen: Bitvector,
}

pub struct CaveParser<'a> {
    smalls: Vec<&'a str>,
}

const START: Cave = Cave(0x0fe);
const END: Cave = Cave(0x0ff);

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

#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd, Debug)]
pub struct Cave(u8);

impl Cave {
    fn small_onehot(&self) -> Bitvector {
        match self {
            Cave(x) => 1 << x,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct CaveMap<T> {
    start: Option<T>,
    end: Option<T>,
    small: [Option<T>; MAX_SMALL],
}

impl<T> CaveMap<T> {
    pub fn new() -> Self {
        CaveMap {
            start: None,
            end: None,
            small: Default::default(),
        }
    }
    pub fn non_end_empty(&self) -> bool {
        self.start.is_none() && self.small.iter().all(|x| x.is_none())
    }
}

impl<T> Index<Cave> for CaveMap<T> {
    type Output = Option<T>;
    fn index(&self, ix: Cave) -> &Option<T> {
        match ix {
            x if x == START => &self.start,
            x if x == END => &self.end,
            Cave(i) => &self.small[i as usize],
        }
    }
}

impl<T> IndexMut<Cave> for CaveMap<T> {
    fn index_mut(&mut self, ix: Cave) -> &mut Option<T> {
        match ix {
            x if x == START => &mut self.start,
            x if x == END => &mut self.end,
            Cave(i) => &mut self.small[i as usize],
        }
    }
}

fn alist_get_or_insert<K: Eq, V>(alist: &mut Vec<(K, V)>, target: K, default: V) -> &mut V {
    match alist.iter().position(|(k, _)| *k == target) {
        Some(i) => &mut alist[i].1,
        None => {
            alist.push((target, default));
            &mut alist.last_mut().unwrap().1
        }
    }
}

pub fn parse<S: AsRef<str>>(input: &[S]) -> CaveMap<SmallVec<[(Cave, usize); SMALLVEC_LEN]>> {
    let mut big_edges = Vec::new();
    let mut parser = CaveParser::new();
    let mut edges_raw = Vec::new();
    for (x, y) in input
        .iter()
        .map(|line| line.as_ref().split_once("-").unwrap())
    {
        match (parser.parse(x), parser.parse(y)) {
            (None, None) => panic!("Cannot connect two big caves"),
            (Some(x), Some(y)) => {
                let k = if x < y { (x, y) } else { (y, x) };
                edges_raw.push((k, 1));
            }
            (None, Some(y)) => alist_get_or_insert(&mut big_edges, x, Vec::new()).push(y),
            (Some(x), None) => alist_get_or_insert(&mut big_edges, y, Vec::new()).push(x),
        }
    }
    for (_, small_caves) in big_edges {
        for (i, &x) in small_caves.iter().enumerate() {
            for &y in small_caves[i..].iter() {
                let k = if x < y { (x, y) } else { (y, x) };
                *alist_get_or_insert(&mut edges_raw, k, 0) += 1;
            }
        }
    }
    let mut edges = CaveMap::new();
    for ((x, y), c) in edges_raw.into_iter() {
        edges[x].get_or_insert(SmallVec::new()).push((y, c));
        if x != y {
            edges[y].get_or_insert(SmallVec::new()).push((x, c));
        }
    }
    edges
}

pub fn solve_inner(
    edges: &CaveMap<SmallVec<[(Cave, usize); SMALLVEC_LEN]>>,
) -> (Instant, Instant, (usize, usize)) {
    let mut caves_count = MAX_SMALL as u8;
    for i in 0..(MAX_SMALL as u8) {
        let k = Cave(i);
        if edges[k].is_none() {
            caves_count = i;
            break;
        }
    }
    let mut small_loops: Vec<usize> = vec![0; 1 << caves_count];
    let mut stack = Vec::new();
    for i in 0..caves_count {
        let k = Cave(i);
        let onehot = 1 << i;
        stack.push(Path {
            head: k,
            seen: onehot,
            weight: 1,
        });
        while let Some(path) = stack.pop() {
            for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
                if neighbor == START {
                    continue;
                }
                if neighbor == END {
                    continue;
                }
                let weight = path.weight * neighbor_weight;
                if neighbor == k {
                    small_loops[path.seen as usize] += weight;
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
    let loops_dfs = Instant::now();
    let mut stack = CaveMap::new();
    let mut start_vec = vec![0; 1 << caves_count];
    start_vec[0] = 1;
    stack[START] = Some(start_vec);

    let mut relevant_caves: Vec<Cave> = (0..caves_count).map(|i| Cave(i)).collect();
    relevant_caves.push(START);
    while !stack.non_end_empty() {
        let mut old_stack = CaveMap::new();
        std::mem::swap(&mut stack, &mut old_stack);
        stack[END] = old_stack[END].take();
        for &head in relevant_caves.iter() {
            let seen_weights = if let Some(sw) = old_stack[head].take() {
                sw
            } else {
                continue;
            };
            for &(neighbor, neighbor_weight) in edges[head].as_ref().unwrap() {
                if neighbor == START {
                    continue;
                }
                let neighbor_onehot = if neighbor == END {
                    0
                } else {
                    neighbor.small_onehot() as usize
                };
                let (target, mut target_empty) = match &mut stack[neighbor] {
                    Some(t) => (t, false),
                    None => {
                        stack[neighbor] = Some(vec![0; 1 << caves_count]);
                        (stack[neighbor].as_mut().unwrap(), true)
                    }
                };
                for (seen, &weight) in seen_weights.iter().enumerate() {
                    if (seen & neighbor_onehot) > 0 {
                        continue;
                    }
                    target_empty &= weight == 0;
                    let new_seen = seen | neighbor_onehot;
                    let new_weight = weight * neighbor_weight;
                    target[new_seen] += new_weight;
                }
                if target_empty {
                    stack[neighbor] = None;
                }
            }
        }
    }
    let paths = stack[END].take().unwrap();
    let paths_dfs = Instant::now();
    let mut one_count = 0;
    let mut count = 0;
    for (path, weight) in paths.into_iter().enumerate() {
        let path = path as Bitvector;
        if weight == 0 {
            continue;
        }
        let mut total_loop_count = 0;
        for (small_loop, &loop_count) in small_loops.iter().enumerate() {
            let small_loop = small_loop as Bitvector;
            if (small_loop & path).is_power_of_two() {
                total_loop_count += loop_count;
            }
        }
        count += weight * (1 + total_loop_count);
        one_count += weight;
    }
    (loops_dfs, paths_dfs, (one_count, count))
}
