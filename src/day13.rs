use std::collections::HashSet;

pub fn solve1(input: &[String]) {
    let mut split = input.split(|line| line == "");
    let coords: HashSet<(usize, usize)> = split
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds: Vec<(&str, usize)> = split
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let (axis, n) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once("=")
                .unwrap();
            (axis, n.parse().unwrap())
        })
        .collect();
    dbg!(apply_fold(coords.clone(), folds[0]).len());
    let folded = folds.into_iter().fold(coords, apply_fold);
    let max_x = folded.iter().map(|(x, y)| x).max().unwrap().clone();
    let max_y = folded.iter().map(|(x, y)| y).max().unwrap().clone();
    for y in 0..=max_y {
        let line: String = (0..=max_x)
            .map(|x| if folded.contains(&(x, y)) { '#' } else { ' ' })
            .collect();
        println!("{}", line)
    }
}

fn apply_fold(
    points: HashSet<(usize, usize)>,
    (axis, n): (&str, usize),
) -> HashSet<(usize, usize)> {
    points
        .into_iter()
        .map(|(x, y)| {
            if axis == "x" && x > n {
                (2 * n - x, y)
            } else if axis == "y" && y > n {
                (x, 2 * n - y)
            } else {
                (x, y)
            }
        })
        .collect()
}
