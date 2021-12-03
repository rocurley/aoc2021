pub fn solve1(input: &[String]) {
    let numbers: Vec<u32> = input
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();
    let gamma = most_common_bits(&numbers);
    let n_bits = input[0].len();
    let epsilon = ((1 << n_bits) - 1) as u32 - gamma;
    println!("{}", epsilon * gamma);

    let mut o2_candidates = numbers.clone();
    for i in (0..n_bits).rev() {
        assert!(o2_candidates.len() > 0);
        let target = (most_common_bits(&o2_candidates) >> i) & 1;
        o2_candidates.retain(|candidate| (candidate >> i) & 1 == target);
        if o2_candidates.len() == 1 {
            break;
        }
    }
    let mut co2_candidates = numbers.clone();
    for i in (0..n_bits).rev() {
        assert!(co2_candidates.len() > 0);
        let target = (most_common_bits(&co2_candidates) >> i) & 1;
        co2_candidates.retain(|candidate| (candidate >> i) & 1 != target);
        if co2_candidates.len() == 1 {
            break;
        }
    }
    println!("{}", o2_candidates[0] * co2_candidates[0]);
}

fn most_common_bits(numbers: &[u32]) -> u32 {
    let mut out = 0;
    for i in 0..32 {
        let count: u32 = numbers.iter().map(|n| (n >> i) & 1).sum();
        if count >= (numbers.len() + 1) as u32 / 2 {
            out += 1 << i;
        }
    }
    out
}
