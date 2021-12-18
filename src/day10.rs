use std::collections::HashMap;
pub fn solve1(input: &[String]) {
    let score: u64 = input
        .iter()
        .filter_map(find_illegal)
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unimplemented!(),
        })
        .sum();
    dbg!(score);
    let mut score2s: Vec<u64> = input
        .iter()
        .filter_map(find_completion)
        .map(|stack| {
            let mut score = 0;
            for c in stack.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unimplemented!(),
                };
            }
            score
        })
        .collect();
    score2s.sort();
    let score2 = score2s[score2s.len() / 2];
    dbg!(score2);
}

fn find_illegal(line: &String) -> Option<char> {
    let pairs: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();
    let mut stack = Vec::new();
    for c in line.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else {
            let other = stack.pop().unwrap();
            if pairs[&other] != c {
                return Some(c);
            }
        }
    }
    return None;
}

fn find_completion(line: &String) -> Option<Vec<char>> {
    let pairs: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();
    let mut stack = Vec::new();
    for c in line.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else {
            let other = stack.pop().unwrap();
            if pairs[&other] != c {
                return None;
            }
        }
    }
    return Some(stack);
}
