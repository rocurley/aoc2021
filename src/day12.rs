use std::collections::{HashMap, HashSet};
pub fn solve1(input: &[String]) {
    let mut edges = HashMap::new();
    for (x, y) in input.iter().map(|line| line.split_once("-").unwrap()) {
        edges.entry(x).or_insert_with(|| Vec::new()).push(y);
        edges.entry(y).or_insert_with(|| Vec::new()).push(x);
    }
    let mut count = 0;
    let mut stack = vec![vec!["start"]];
    while let Some(path) = stack.pop() {
        for neighbor in &edges[path.last().unwrap()] {
            if neighbor == &"end" {
                count += 1;
                continue;
            }
            let is_small = neighbor.chars().next().unwrap().is_lowercase();
            if is_small && path.contains(neighbor) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(neighbor);
            stack.push(new_path);
        }
    }
    dbg!(count);

    let mut small_loops: HashMap<Vec<&str>, usize> = HashMap::new();
    for k in edges
        .keys()
        .filter(|k| *k != &"start" && *k != &"end" && k.chars().next().unwrap().is_lowercase())
    {
        let mut stack = vec![vec![*k]];
        while let Some(path) = stack.pop() {
            for neighbor in &edges[path.last().unwrap()] {
                if neighbor == &"start" {
                    continue;
                }
                if neighbor == &"end" {
                    continue;
                }
                if neighbor == k {
                    let mut ix: Vec<&str> = path.iter().copied().filter(|x| is_small(x)).collect();
                    ix.sort();
                    *small_loops.entry(ix).or_insert(0) += 1;
                    continue;
                }
                let is_small = neighbor.chars().next().unwrap().is_lowercase();
                if is_small && path.contains(neighbor) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(neighbor);
                stack.push(new_path);
            }
        }
    }
    for (k, v) in small_loops.iter_mut() {
        *v /= k.len();
    }
    dbg!(&small_loops);
    let mut count = 0;
    let mut stack = vec![vec!["start"]];
    while let Some(path) = stack.pop() {
        for neighbor in &edges[path.last().unwrap()] {
            if neighbor == &"start" {
                continue;
            }
            if neighbor == &"end" {
                count += 1;
                for (small_loop, loop_count) in small_loops.iter() {
                    if small_loop.iter().filter(|k| path.contains(k)).count() == 1 {
                        count += loop_count;
                    }
                }
                continue;
            }
            let is_small = is_small(neighbor);
            if is_small && path.contains(neighbor) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(neighbor);
            stack.push(new_path);
        }
    }
    dbg!(count);
}

fn is_small(k: &str) -> bool {
    k != "start" && k != "end" && k.chars().next().unwrap().is_lowercase()
}
