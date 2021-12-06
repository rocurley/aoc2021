pub fn solve1(input: &[String]) {
    let numbers: Vec<i32> = input[0]
        .split(",")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    let boards: Vec<Vec<Vec<i32>>> = input[1..]
        .chunks(6)
        .map(|raw| {
            raw[1..]
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| i32::from_str_radix(x, 10).unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    let rows: Vec<Vec<(usize, usize)>> = (0..5)
        .into_iter()
        .map(|x| (0..5).map(|y| (x, y)).collect())
        .collect();
    let cols: Vec<Vec<(usize, usize)>> = rows
        .iter()
        .map(|row| row.iter().copied().map(|(x, y)| (y, x)).collect())
        .collect();
    let diagonals = vec![
        vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)],
        vec![(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)],
    ];
    let possible_wins = [rows, cols].concat();
    let mut marked: Vec<Vec<(usize, usize)>> = vec![Vec::new(); boards.len()];
    let mut has_won = vec![false; boards.len()];
    for target in numbers {
        for ((board, board_marked), has_won_board) in
            boards.iter().zip(marked.iter_mut()).zip(has_won.iter_mut())
        {
            for (y, row) in board.iter().enumerate() {
                for (x, &n) in row.iter().enumerate() {
                    if n == target {
                        board_marked.push((x, y));
                    }
                }
            }
            if is_win(board_marked, &possible_wins) && !*has_won_board {
                *has_won_board = true;
                let score: i32 = board
                    .iter()
                    .enumerate()
                    .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, val)| ((x, y), val)))
                    .filter(|(pt, val)| !board_marked.contains(pt))
                    .map(|(pt, val)| val)
                    .sum();
                println!("{}", score * target);
            }
        }
    }
}

fn is_win(marked: &[(usize, usize)], possible_wins: &[Vec<(usize, usize)>]) -> bool {
    possible_wins.iter().any(|possible_win| {
        let res = possible_win.iter().all(|pt| marked.contains(pt));
        res
    })
}
