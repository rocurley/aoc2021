use std::collections::{BinaryHeap, HashSet};
pub fn solve1(input: &[String]) {
    let map: Vec<Vec<u32>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    solve(map.clone());
    let map_wide: Vec<Vec<u32>> = map
        .iter()
        .map(|row| {
            (0..5)
                .into_iter()
                .flat_map(|i| row.iter().copied().map(move |x| (x + i - 1) % 9 + 1))
                .collect()
        })
        .collect();
    let map_tall: Vec<Vec<u32>> = (0..5)
        .into_iter()
        .flat_map(|i| {
            map_wide.iter().map(move |row| {
                row.iter()
                    .copied()
                    .map(move |x| (x + i - 1) % 9 + 1)
                    .collect()
            })
        })
        .collect();
    solve(map_tall);
}

fn solve(map: Vec<Vec<u32>>) {
    let mut heap = BinaryHeap::new();
    let xmax = map[0].len() - 1;
    let ymax = map.len() - 1;
    heap.push(HeapElem {
        pt: (0, 0),
        cost: 0,
    });
    let mut seen = HashSet::new();
    let cost = loop {
        let HeapElem { pt, cost } = heap.pop().unwrap();
        if seen.contains(&pt) {
            continue;
        }
        seen.insert(pt);
        if pt.0 == xmax && pt.1 == ymax {
            break cost;
        }
        for n_pt in neighbors(pt, xmax, ymax) {
            let new = HeapElem {
                cost: cost + map[n_pt.1][n_pt.0],
                pt: n_pt,
            };
            heap.push(new);
        }
    };
    dbg!(cost);
}

struct HeapElem {
    pt: (usize, usize),
    cost: u32,
}
impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapElem {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_cost() == other.heuristic_cost()
    }
}
impl Eq for HeapElem {}

impl HeapElem {
    fn heuristic_cost(&self) -> i32 {
        self.cost as i32 - self.pt.0 as i32 - self.pt.1 as i32
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic_cost().cmp(&self.heuristic_cost())
    }
}

fn neighbors(
    (x, y): (usize, usize),
    xmax: usize,
    ymax: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        if x + 1 <= xmax {
            Some((x + 1, y))
        } else {
            None
        },
        if y + 1 <= ymax {
            Some((x, y + 1))
        } else {
            None
        },
        if x > 0 { Some((x - 1, y)) } else { None },
        if y > 0 { Some((x, y - 1)) } else { None },
    ]
    .into_iter()
    .filter_map(|x| x)
}
