pub fn solve1(input: &[String]) {
    let numbers: Vec<Vec<u32>> = input
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let gamma_bits = most_common_bits(&numbers);
    let gamma: u32 = bits_to_number(&gamma_bits);
    let epsilon = ((1 << gamma_bits.len()) - 1) as u32 - gamma;
    println!("{}", epsilon * gamma);

    let mut o2_candidates = numbers.clone();
    for i in 0.. {
        assert!(o2_candidates.len() > 0);
        let target = most_common_bits(&o2_candidates)[i];
        o2_candidates.retain(|candidate| candidate[i] == target);
        if o2_candidates.len() == 1 {
            break;
        }
    }
    let mut co2_candidates = numbers.clone();
    for i in 0.. {
        assert!(co2_candidates.len() > 0);
        let target = most_common_bits(&co2_candidates)[i];
        co2_candidates.retain(|candidate| candidate[i] != target);
        if co2_candidates.len() == 1 {
            break;
        }
    }
    println!(
        "{}",
        bits_to_number(&o2_candidates[0]) * bits_to_number(&co2_candidates[0])
    );
}
fn bits_to_number(xs: &[u32]) -> u32 {
    xs.iter()
        .rev()
        .enumerate()
        .map(|(i, s)| (1 << i as u32) * s)
        .sum()
}

fn most_common_bits(numbers: &[Vec<u32>]) -> Vec<u32> {
    let mut sum = vec![0; numbers[0].len()];
    for n in numbers.iter() {
        for (d, s_d) in n.iter().zip(sum.iter_mut()) {
            *s_d += d;
        }
    }
    sum.iter()
        .map(|s| std::cmp::min(1, s * 2 / numbers.len() as u32))
        .collect()
}
