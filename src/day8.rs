struct Display {
    patterns: Vec<String>,
    display: Vec<String>,
}

pub fn solve1(input: &[String]) {
    let displays: Vec<Display> = input
        .iter()
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            Display {
                patterns: left.split(" ").map(|segs| segs.chars().collect()).collect(),
                display: right
                    .split(" ")
                    .map(|segs| segs.chars().collect())
                    .collect(),
            }
        })
        .collect();
    let uniques = displays
        .iter()
        .flat_map(|display| display.display.iter())
        .filter(|segs| [2, 3, 4, 7].contains(&segs.len()))
        .count();
    println!("{}", uniques);
    let number_patterns = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let mut out = 0;
    for display in displays {
        let permutation = permutations(8)
            .find(|permutation| check_permutation(&display.patterns, permutation, &number_patterns))
            .unwrap();
        let digits: usize = display
            .display
            .iter()
            .map(|s| {
                let permuted = sort_string(&apply_permuation(&permutation, s));
                number_patterns.iter().position(|n| n == &permuted).unwrap()
            })
            .enumerate()
            .map(|(i, n)| 10_usize.pow(3 - i as u32) * n)
            .sum();
        out += digits;
    }
    println!("{}", out);
}

fn sort_string(unsorted: &str) -> String {
    let mut s_vec: Vec<char> = unsorted.chars().collect();
    s_vec.sort();
    s_vec.iter().collect()
}

fn permutations(n: u8) -> Box<dyn Iterator<Item = Vec<u8>>> {
    if n == 0 {
        Box::new(std::iter::once(Vec::new()))
    } else {
        Box::new(permutations(n - 1).flat_map(move |perms| {
            (0..n).map(move |x| {
                let mut out = perms.clone();
                out.insert(x as usize, n - 1);
                out
            })
        }))
    }
}

fn apply_permuation(permutation: &[u8], chars: &str) -> String {
    chars
        .chars()
        .map(|c| (permutation[c as usize - 'a' as usize] + 'a' as u8) as char)
        .collect()
}

fn check_permutation(patterns: &[String], permutation: &[u8], valid: &[&str]) -> bool {
    patterns.iter().all(|pattern| {
        let unsorted = apply_permuation(permutation, pattern);
        let mut s_vec: Vec<char> = unsorted.chars().collect();
        s_vec.sort();
        let sorted: String = s_vec.iter().collect();
        valid.contains(&sorted.as_str())
    })
}
