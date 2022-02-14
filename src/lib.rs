use arrayvec::ArrayVec;
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
type Edges = CaveMap<ArrayVec<(Cave, u32), SMALLVEC_LEN>>;
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
    small: ArrayVec<T, MAX_SMALL>,
}

impl<T: Clone> CaveMap<T> {
    fn new_cloned(init: T, n_caves: u8) -> Self {
        let small = vec![init.clone(); n_caves as usize].into_iter().collect();
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

fn alist_get_or_insert<K: Eq, V>(alist: &mut Vec<(K, V)>, target: K, default: V) -> &mut V {
    match alist.iter().position(|(k, _)| *k == target) {
        Some(i) => &mut alist[i].1,
        None => {
            alist.push((target, default));
            &mut alist.last_mut().unwrap().1
        }
    }
}

fn array_alist_get_or_insert<K: Eq, V, const N: usize>(
    alist: &mut ArrayVec<(K, V), N>,
    target: K,
    default: V,
) -> &mut V {
    match alist.iter().position(|(k, _)| *k == target) {
        Some(i) => &mut alist[i].1,
        None => {
            alist.push((target, default));
            &mut alist.last_mut().unwrap().1
        }
    }
}

pub fn parse<S: AsRef<str>>(input: &[S]) -> Edges {
    let mut big_edges = Vec::new();
    let mut edges = CaveMap::new_cloned(ArrayVec::new(), MAX_SMALL as u8);
    let mut parser = CaveParser::new();
    for (x, y) in input
        .iter()
        .map(|line| line.as_ref().split_once("-").unwrap())
    {
        match (parser.parse(x), parser.parse(y)) {
            (None, None) => panic!("Cannot connect two big caves"),
            (Some(x), Some(y)) => {
                edges[x].push((y, 1));
                edges[y].push((x, 1));
            }
            (None, Some(y)) => alist_get_or_insert(&mut big_edges, x, Vec::new()).push(y),
            (Some(x), None) => alist_get_or_insert(&mut big_edges, y, Vec::new()).push(x),
        }
    }
    let n_small_caves = parser.smalls.len();
    edges.small.truncate(n_small_caves);
    for (_, small_caves) in big_edges {
        for (i, &x) in small_caves.iter().enumerate() {
            for &y in small_caves[i..].iter() {
                *array_alist_get_or_insert(&mut edges[x], y, 0) += 1;
                if x != y {
                    *array_alist_get_or_insert(&mut edges[y], x, 0) += 1;
                }
            }
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

pub fn find_paths_and_join(edges: &Edges, small_loops: &PathWeights) -> (u32, u32) {
    let path = MiniPath {
        head: START,
        seen: 0,
    };
    let mut cache = CaveMap::new_cloned(vec![None; 1 << edges.count()], edges.count());
    find_paths_and_join_inner(edges, small_loops, path, &mut cache)
}

#[derive(PartialEq, Eq, Hash)]
pub struct MiniPath {
    head: Cave,
    seen: Bitvector,
}

fn find_paths_and_join_inner(
    edges: &Edges,
    small_loops: &PathWeights,
    path: MiniPath,
    cache: &mut CaveMap<Vec<Option<(u32, u32)>>>,
) -> (u32, u32) {
    if let Some(out) = cache[path.head][path.seen as usize] {
        return out;
    }
    if path.head == END {
        let mut intersecting_loop_count = 0;
        for (small_loop, loop_count) in small_loops.iter().enumerate() {
            if (small_loop as u16 & path.seen).count_ones() == 1 {
                intersecting_loop_count += loop_count;
            }
        }
        let count = intersecting_loop_count + 1;
        return (1, count);
    }
    let mut one_count = 0;
    let mut count = 0;
    for &(neighbor, neighbor_weight) in &edges[path.head] {
        if neighbor == START {
            continue;
        }
        let child = if neighbor == END {
            MiniPath {
                head: END,
                seen: path.seen,
            }
        } else {
            let neighbor_onehot = neighbor.onehot();
            if path.seen & neighbor_onehot > 0 {
                continue;
            }
            MiniPath {
                head: neighbor,
                seen: path.seen | neighbor_onehot,
            }
        };
        let (child_one_count, child_count) =
            find_paths_and_join_inner(edges, small_loops, child, cache);
        one_count += neighbor_weight * child_one_count;
        count += neighbor_weight * child_count;
    }
    let out = (one_count, count);
    cache[path.head][path.seen as usize] = Some(out);
    out
}

pub fn solve(edges: &Edges) -> (u32, u32) {
    let small_loops = find_small_loops(edges);
    find_paths_and_join(edges, &small_loops)
}
