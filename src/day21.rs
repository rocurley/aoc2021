use std::collections::HashMap;
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
    let mut p1 = PlayerWorlds {
        weight: 1,
        wins: 0,
        worlds: p1_worlds,
    };
    let mut p2 = PlayerWorlds {
        weight: 1,
        wins: 0,
        worlds: p2_worlds,
    };
    loop {
        let mut new_p1 = PlayerWorlds {
            weight: 0,
            wins: p1.wins,
            worlds: HashMap::new(),
        };
        for (p1, weight) in p1.worlds {
            let positions = roll_dice(roll_dice(roll_dice([p1.position].into_iter())));
            for position in positions {
                let new_p1_world = Player {
                    position,
                    score: p1.score + position,
                };
                if new_p1_world.score >= 21 {
                    new_p1.wins += weight * p2.weight;
                } else {
                    new_p1.weight += weight;
                    *new_p1.worlds.entry(new_p1_world).or_insert(0) += weight;
                }
            }
        }
        p1 = new_p1;
        if p1.worlds.is_empty() {
            break;
        }
        std::mem::swap(&mut p1, &mut p2);
    }
    dbg!(p1.wins, p2.wins);
}

fn roll_dice(ps: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    ps.flat_map(|p| (1..=3).map(move |roll| (p + roll - 1) % 10 + 1))
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Player {
    position: u32,
    score: u32,
}

#[derive(Clone, Debug)]
struct PlayerWorlds {
    weight: u64,
    wins: u64,
    worlds: HashMap<Player, u64>,
}
