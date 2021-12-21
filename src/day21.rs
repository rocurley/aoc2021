use std::collections::HashMap;
use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let mut p1 = Player {
        position: 1,
        score: 0,
    };
    let mut p2 = Player {
        position: 6,
        score: 0,
    };
    let mut next_roll = 1;
    let mut rolls = 0;
    loop {
        for _ in 0..3 {
            rolls += 1;
            p1.position = (p1.position + next_roll - 1) % 10 + 1;
            next_roll = next_roll % 100 + 1;
        }
        p1.score += p1.position;
        if p1.score >= 1000 {
            break;
        }
        for _ in 0..3 {
            rolls += 1;
            p2.position = (p2.position + next_roll - 1) % 10 + 1;
            next_roll = next_roll % 100 + 1;
        }
        p2.score += p2.position;
        if p2.score >= 1000 {
            break;
        }
    }
    dbg!(p1.score, p2.score, rolls);
}
pub fn solve2(input: &[String]) {
    let mut p1_worlds = HashMap::new();
    p1_worlds.insert(
        Player {
            position: 1,
            score: 0,
        },
        1u64,
    );
    let mut p2_worlds = HashMap::new();
    p2_worlds.insert(
        Player {
            position: 6,
            score: 0,
        },
        1u64,
    );
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut p1_weight = 1;
    let mut p2_weight = 1;
    loop {
        let mut new_p1_worlds = HashMap::new();
        let mut new_p1_weight = 0;
        for (p1, weight) in p1_worlds {
            let mut positions = vec![p1.position];
            for _ in 0..3 {
                positions = positions
                    .into_iter()
                    .flat_map(|p| (1..=3).map(move |roll| (p + roll - 1) % 10 + 1))
                    .collect();
            }
            for position in positions {
                let new_p1 = Player {
                    position,
                    score: p1.score + position,
                };
                if new_p1.score >= 21 {
                    p1_wins += weight * p2_weight;
                } else {
                    new_p1_weight += weight;
                    *new_p1_worlds.entry(new_p1).or_insert(0) += weight;
                }
            }
        }
        p1_worlds = new_p1_worlds;
        p1_weight = new_p1_weight;
        if p1_worlds.is_empty() {
            break;
        }
        std::mem::swap(&mut p1_wins, &mut p2_wins);
        std::mem::swap(&mut p1_weight, &mut p2_weight);
        std::mem::swap(&mut p1_worlds, &mut p2_worlds);
    }
    dbg!(p1_wins, p2_wins);
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Player {
    position: u32,
    score: u32,
}
