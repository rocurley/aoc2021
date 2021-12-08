pub fn solve1(input: &[String]) {
    let mut fish: [usize; 9] = [0; 9];
    for age in input[0].split(",") {
        fish[age.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..80 {
        step_fish(&mut fish);
    }
    println!("{}", fish.iter().copied().sum::<usize>());
    for _ in 80..256 {
        step_fish(&mut fish);
    }
    println!("{}", fish.iter().copied().sum::<usize>());
}

fn step_fish(fish: &mut [usize; 9]) {
    fish.rotate_left(1);
    fish[6] += fish[8];
}
