use std::collections::HashSet;

pub fn solve1(input: &[String]) {
    let map: Vec<Vec<u32>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let low_points = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, h)| ((i, j), h)))
        .filter(|(pt, h)| {
            neighbors(*pt)
                .iter()
                .filter_map(|(ni, nj)| map.get(*ni).and_then(|row| row.get(*nj)))
                .all(|nh| nh > h)
        });
    // let out: u32 = low_points.map(|(_, h)| 1 + h).sum();
    // dbg!(out);
    let mut basins: Vec<usize> = low_points.map(|(pt, _)| flood_size(&map, pt)).collect();
    dbg!(&basins);
    basins.sort_by_key(|x| -(*x as i64));
    dbg!(basins[0] * basins[1] * basins[2]);
}

fn neighbors((i, j): (usize, usize)) -> [(usize, usize); 4] {
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
}

fn flood_size(map: &[Vec<u32>], pt: (usize, usize)) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut stack = vec![pt];
    let mut out = 0;
    while let Some((i, j)) = stack.pop() {
        if seen.contains(&(i, j)) {
            continue;
        }
        seen.insert((i, j));
        let h = map
            .get(i)
            .and_then(|line| line.get(j))
            .copied()
            .unwrap_or(9);
        if h == 9 {
            continue;
        }
        out += 1;
        for n in neighbors((i, j)) {
            stack.push(n);
        }
    }
    out
}
