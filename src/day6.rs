use std::collections::HashMap;

pub fn solve1(input: &[String]) {
    let mut fish: [usize; 9] = [0; 9];
    for age in input[0].split(",") {
        fish[age.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..80 {
        let mut new_fish: [usize; 9] = [0; 9];
        new_fish[0..8].copy_from_slice(&fish[1..9]);
        new_fish[8] = fish[0];
        new_fish[6] += fish[0];
        fish = new_fish;
    }
    println!("{}", fish.iter().copied().sum::<usize>());
    for _ in 80..256 {
        let mut new_fish: [usize; 9] = [0; 9];
        new_fish[0..8].copy_from_slice(&fish[1..9]);
        new_fish[8] = fish[0];
        new_fish[6] += fish[0];
        fish = new_fish;
    }
    println!("{}", fish.iter().copied().sum::<usize>());
}
