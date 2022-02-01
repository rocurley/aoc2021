use smallvec::SmallVec;
use std::default::Default;
use std::ops::{Index, IndexMut};
use std::simd::{mask32x16, u32x16};
use std::time::Instant;

pub fn solve1(input: &[String]) {
    let start_t = Instant::now();
    let (edges, caves_count) = parse(input);
    let parsing = Instant::now();
    let (loops_dfs, paths_dfs, (sol1, sol2)) = solve_inner(&edges, caves_count);
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

type WeightsVec = u32x16;
type WeightsMask = mask32x16;
const WEIGHTS_LANES: usize = WeightsVec::LANES;
const WEIGHTS_LANE_WIDTH: usize = std::mem::size_of::<WeightsVec>() / WEIGHTS_LANES;

pub struct Path {
    head: Cave,
    seen: Bitvector,
    weight: u32,
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

type Edges = CaveMap<SmallVec<[(Cave, u32); SMALLVEC_LEN]>>;

pub fn parse<S: AsRef<str>>(input: &[S]) -> (Edges, u8) {
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
    (edges, parser.smalls.len() as u8)
}

pub fn solve_inner(edges: &Edges, caves_count: u8) -> (Instant, Instant, (usize, usize)) {
    let small_loops = find_loops(edges, caves_count);
    let loops_dfs = Instant::now();
    let paths = find_paths(edges, caves_count);
    let paths_dfs = Instant::now();
    let mut one_count = 0;
    let mut count = 0;
    for (path_top_bits, weight_vec) in paths.into_iter().enumerate() {
        for (path_lower_bits, &weight) in weight_vec.as_array().into_iter().enumerate() {
            if weight == 0 {
                continue;
            }
            let path = (path_top_bits * WEIGHTS_LANES | path_lower_bits) as Bitvector;
            let mut total_loop_count = 0;
            for (small_loop, &loop_count) in small_loops.iter().enumerate() {
                let small_loop = small_loop as Bitvector;
                if (small_loop & path).is_power_of_two() {
                    total_loop_count += loop_count as usize;
                }
            }
            count += weight as usize * (1 + total_loop_count);
            one_count += weight;
        }
    }
    (loops_dfs, paths_dfs, (one_count as usize, count))
}

fn find_loops(edges: &Edges, caves_count: u8) -> Vec<u32> {
    let mut small_loops: Vec<u32> = vec![0; 1 << caves_count];
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
    small_loops
}

pub fn find_paths(edges: &Edges, caves_count: u8) -> Vec<WeightsVec> {
    assert!((1 << caves_count) >= WEIGHTS_LANES);
    let mut stack = CaveMap::new();
    let n_vectors = (1 << caves_count) / WEIGHTS_LANES;
    let mut start_vec = vec![WeightsVec::splat(0); n_vectors];
    start_vec[0][0] = 1;
    stack[START] = Some(start_vec);

    let mut relevant_caves: Vec<Cave> = (0..caves_count).map(|i| Cave(i)).collect();
    relevant_caves.push(START);
    while !stack.non_end_empty() {
        for &head in relevant_caves.iter() {
            let seen_weights = if let Some(sw) = stack[head].take() {
                sw
            } else {
                continue;
            };
            for &(neighbor, neighbor_weight) in edges[head].as_ref().unwrap() {
                if neighbor == START {
                    continue;
                }
                if neighbor == head {
                    continue;
                }
                let target_maybe_empty = stack[neighbor].is_none();
                let target =
                    stack[neighbor].get_or_insert_with(|| vec![WeightsVec::splat(0); n_vectors]);
                if neighbor == END {
                    let neighbor_weight = WeightsVec::splat(neighbor_weight as u32);
                    for (weight, target_weight) in seen_weights.iter().zip(target.iter_mut()) {
                        *target_weight += neighbor_weight * weight;
                    }
                    continue;
                }
                let shift = neighbor.small_onehot() as usize;
                match shift {
                    1 => {
                        subvector_bfs_step::<1>(neighbor_weight as u32, &seen_weights, target);
                    }
                    2 => {
                        subvector_bfs_step::<2>(neighbor_weight as u32, &seen_weights, target);
                    }
                    4 => {
                        subvector_bfs_step::<4>(neighbor_weight as u32, &seen_weights, target);
                    }
                    8 => {
                        subvector_bfs_step::<8>(neighbor_weight as u32, &seen_weights, target);
                    }
                    _ => {
                        assert!(shift >= WEIGHTS_LANES);
                        let neighbor_weight = WeightsVec::splat(neighbor_weight as u32);
                        let shift_vecs = shift / WEIGHTS_LANES;
                        for (seen, &weight) in seen_weights.iter().enumerate() {
                            if (seen / shift_vecs) % 2 == 1 {
                                continue;
                            }
                            target[seen + shift_vecs] += weight * neighbor_weight;
                        }
                    }
                };
                if target_maybe_empty {
                    let zero = WeightsVec::splat(0);
                    let lanes_zero = target
                        .iter()
                        .fold(WeightsMask::splat(true), |acc, v| acc & v.lanes_eq(zero));
                    if lanes_zero.all() {
                        stack[neighbor] = None;
                    }
                }
            }
        }
    }
    stack[END].take().unwrap()
}

fn subvector_bfs_step<const SHIFT: usize>(
    neighbor_weight: u32,
    seen_weights: &[WeightsVec],
    target: &mut [WeightsVec],
) {
    let mut neighbor_weight_mask = WeightsVec::splat(neighbor_weight as u32);
    for i in 0..WEIGHTS_LANES {
        if (i / SHIFT) % 2 == 1 {
            neighbor_weight_mask[i] = 0;
        }
    }
    for (weight, target_weight) in seen_weights.iter().zip(target.iter_mut()) {
        let masked_weight = (weight * neighbor_weight_mask).rotate_lanes_right::<SHIFT>();
        *target_weight += masked_weight;
    }
}

pub fn find_paths_ref(edges: &Edges, caves_count: u8) -> Vec<u32> {
    let mut stack = vec![Path {
        head: START,
        seen: 0,
        weight: 1,
    }];
    let mut paths: Vec<u32> = vec![0; 1 << caves_count];
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in edges[path.head].as_ref().unwrap() {
            if neighbor == START {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == END {
                paths[path.seen as usize] += weight as u32;
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
    paths
}

fn cave_str(cave: Cave) -> String {
    if cave == START {
        "start".to_owned()
    } else if cave == END {
        "end".to_owned()
    } else {
        format!("{}", cave.0)
    }
}

fn edges_to_dot(edges: &Edges) {
    println!("graph {{");
    println!("overlap = false;");
    println!("splines=true;");
    println!("sep=\"+25,25\";");
    println!("overlap=scalexy;");
    let mut relevant_caves: Vec<Cave> = (0..MAX_SMALL as u8).map(|i| Cave(i)).collect();
    relevant_caves.push(START);
    relevant_caves.push(END);
    for head in relevant_caves {
        if let Some(head_edges) = edges[head].as_ref() {
            for &(tail, weight) in head_edges {
                if head >= tail {
                    println!("{}--{} [label={}]", cave_str(head), cave_str(tail), weight);
                }
            }
        }
    }
    println!("}}");
}

mod test {
    #[cfg(test)]
    const INPUT: [&'static str; 13] = [
        "DG-dc", "start-dc", "start-ah", "MN-start", "XB-fi", "MN-ah", "MN-dc", "fi-end", "th-fi",
        "end-XB", "dc-XB", "dc-ah", "MN-fi",
    ];

    #[test]
    fn test_find_paths() {
        let (edges, caves_count) = super::parse(&INPUT);
        super::edges_to_dot(&edges);
        let expected = super::find_paths_ref(&edges, caves_count);
        //assert_ne!(0u32, expected.into_iter().sum());
        let actual = super::find_paths(&edges, caves_count);
        let actual_flattened: Vec<u32> = actual
            .into_iter()
            .flat_map(super::WeightsVec::to_array)
            .collect();
        assert_eq!(expected, actual_flattened);
    }
}
