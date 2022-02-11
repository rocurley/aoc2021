use std::collections::HashMap;

type Edges<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

pub fn parse<'a, S: AsRef<str>>(input: &'a [S]) -> Edges<'a> {
    let mut big_edges = HashMap::new();
    let mut edges_raw = HashMap::new();
    for (x, y) in input
        .iter()
        .map(|line| line.as_ref().split_once("-").unwrap())
    {
        match (is_small(x), is_small(y)) {
            (false, false) => panic!("Cannot connect two big caves"),
            (true, true) => {
                let k = if x < y { (x, y) } else { (y, x) };
                assert!(edges_raw.insert(k, 1).is_none());
            }
            (false, true) => big_edges.entry(x).or_insert(Vec::new()).push(y),
            (true, false) => big_edges.entry(y).or_insert(Vec::new()).push(x),
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
    let mut edges = HashMap::new();
    for ((x, y), c) in edges_raw.into_iter() {
        edges.entry(x).or_insert(Vec::new()).push((y, c));
        if x != y {
            edges.entry(y).or_insert(Vec::new()).push((x, c));
        }
    }
    edges
}

pub struct Path<'a> {
    trail: Vec<&'a str>,
    weight: usize,
}

impl<'a> Path<'a> {
    fn head(&self) -> &'a str {
        self.trail.last().unwrap()
    }
}

pub fn solve(edges: &Edges) -> (usize, usize) {
    let mut small_loops: HashMap<Vec<&str>, usize> = HashMap::new();
    for &k in edges.keys().filter(|k| *k != &"start" && *k != &"end") {
        let mut stack = vec![Path {
            trail: vec![k],
            weight: 1,
        }];
        while let Some(path) = stack.pop() {
            for &(neighbor, neighbor_weight) in &edges[path.head()] {
                if neighbor == "start" {
                    continue;
                }
                if neighbor == "end" {
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
    for (k, v) in small_loops.iter_mut() {
        *v /= k.len();
    }
    let mut stack = vec![Path {
        trail: vec!["start"],
        weight: 1,
    }];
    let mut paths: HashMap<Vec<&str>, usize> = HashMap::new();
    while let Some(path) = stack.pop() {
        for &(neighbor, neighbor_weight) in &edges[path.head()] {
            if neighbor == "start" {
                continue;
            }
            let weight = path.weight * neighbor_weight;
            if neighbor == "end" {
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

fn is_small(k: &str) -> bool {
    k.chars().next().unwrap().is_lowercase()
}
