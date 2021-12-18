use std::collections::HashSet;

pub fn solve1(input: &[String]) {
    let mut board: Vec<Vec<u32>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut out = 0;
    for _ in 0..100 {
        out += step(&mut board);
    }
    print_board(&board);
    dbg!(out);
    let count = board.len() * board[0].len();
    for i in 100.. {
        if step(&mut board) == count {
            dbg!(i + 1);
            return;
        }
    }
}

fn step(board: &mut [Vec<u32>]) -> usize {
    for line in board.iter_mut() {
        for x in line.iter_mut() {
            *x += 1;
        }
    }
    let mut stack: Vec<(usize, usize)> = board
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, x)| **x > 9)
                .map(move |(j, _)| (i, j))
        })
        .collect();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    while let Some((i, j)) = stack.pop() {
        if seen.contains(&(i, j)) {
            continue;
        }
        seen.insert((i, j));
        for n_pt in neighbors((i, j)) {
            let n = if let Some(b) = get_board(n_pt, board) {
                b
            } else {
                continue;
            };
            *n += 1;
            if *n > 9 {
                stack.push((n_pt.0 as usize, n_pt.1 as usize));
            }
        }
    }
    for (i, j) in seen.iter().copied() {
        board[i][j] = 0;
    }
    seen.len()
}

fn get_board(pt: (isize, isize), board: &mut [Vec<u32>]) -> Option<&mut u32> {
    let i: usize = pt.0.try_into().ok()?;
    let j: usize = pt.1.try_into().ok()?;
    board
        .get_mut(i)
        .and_then(|line: &mut Vec<u32>| line.get_mut(j))
}

fn neighbors(pt: (usize, usize)) -> impl Iterator<Item = (isize, isize)> {
    (-1..=1).flat_map(move |dx| (-1..=1).map(move |dy| (pt.0 as isize + dx, pt.1 as isize + dy)))
}

fn print_board(board: &[Vec<u32>]) {
    for line in board {
        let line_str: String = line
            .iter()
            .map(|d| {
                if *d > 9 {
                    "#".to_string()
                } else {
                    d.to_string()
                }
            })
            .collect();
        println!("{}", line_str);
    }
}
