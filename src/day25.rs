pub fn solve1(input: &[String]) {
    let mut state: Vec<Vec<Option<Cucumber>>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(move |c| match c {
                    '.' => None,
                    'v' => Some(Cucumber::Down),
                    '>' => Some(Cucumber::Right),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    for i in 1.. {
        let next = evolve(&state);
        if state == next {
            println!("{}", i);
            break;
        }
        state = next;
    }
}

fn evolve(prior: &Vec<Vec<Option<Cucumber>>>) -> Vec<Vec<Option<Cucumber>>> {
    let h = prior.len();
    let w = prior[0].len();
    let mut out: Vec<Vec<Option<Cucumber>>> = vec![vec![None; prior[0].len()]; prior.len()];
    for (i, prior_row) in prior.iter().enumerate() {
        for (j, prior_cell) in prior_row.iter().enumerate() {
            if let Some(Cucumber::Right) = prior_cell {
                if prior[i][(j + 1) % w].is_none() {
                    out[i][(j + 1) % w] = Some(Cucumber::Right);
                } else {
                    out[i][j] = Some(Cucumber::Right);
                }
            }
        }
    }
    for (i, prior_row) in prior.iter().enumerate() {
        for (j, prior_cell) in prior_row.iter().enumerate() {
            if let Some(Cucumber::Down) = prior_cell {
                if prior[(i + 1) % h][j] != Some(Cucumber::Down) && out[(i + 1) % h][j].is_none() {
                    out[(i + 1) % h][j] = Some(Cucumber::Down);
                } else {
                    out[i][j] = Some(Cucumber::Down);
                }
            }
        }
    }
    out
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Cucumber {
    Right,
    Down,
}

fn print(state: &Vec<Vec<Option<Cucumber>>>) {
    for line in state {
        for cell in line {
            print!(
                "{}",
                match cell {
                    None => '.',
                    Some(Cucumber::Down) => 'v',
                    Some(Cucumber::Right) => '>',
                }
            );
        }
        println!("");
    }
}
