pub fn solve1(input: &[String]) {
    let mut fish: [usize; 9] = [0; 9];
    for age in input[0].split(",") {
        fish[age.parse::<usize>().unwrap()] += 1;
    }
    dbg!(fish);
    step_fish_fast_1(256, &mut fish);
    let out = fish.iter().copied().sum::<usize>();
    assert_eq!(out, 1572358335990);
    println!("{}", out);
}

pub fn step_fish(fish: &mut [usize; 9]) {
    fish.rotate_left(1);
    fish[6] += fish[8];
}

pub fn step_fish_fast_1(n: usize, fish: &mut [usize; 9]) {
    for i in 0..n {
        fish[(7 + i) % 9] += fish[i % 9];
    }
}

pub fn step_fish_fast_2(n: usize, fish: &mut [usize; 9]) {
    for _ in 0..n / 9 {
        fish[7] += fish[0];
        fish[8] += fish[1];
        fish[0] += fish[2];
        fish[1] += fish[3];
        fish[2] += fish[4];
        fish[3] += fish[5];
        fish[4] += fish[6];
        // First data dependency
        fish[5] += fish[7];
        fish[6] += fish[8];
    }
    for i in 0..n % 9 {
        fish[(7 + i) % 9] += fish[i % 9];
    }
}

pub fn step_fish_fast_3(mut n: usize, fish: &mut [usize; 9]) {
    while n >= 9 {
        fish[7] += fish[0];
        fish[8] += fish[1];
        fish[0] += fish[2];
        fish[1] += fish[3];
        fish[2] += fish[4];
        fish[3] += fish[5];
        fish[4] += fish[6];
        // First data dependency
        fish[5] += fish[7];
        fish[6] += fish[8];
        n -= 9
    }
    if n == 0 {
        return;
    }
    fish[7] += fish[0];
    if n == 1 {
        return;
    }
    fish[8] += fish[1];
    if n == 2 {
        return;
    }
    fish[0] += fish[2];
    if n == 3 {
        return;
    }
    fish[1] += fish[3];
    if n == 4 {
        return;
    }
    fish[2] += fish[4];
    if n == 5 {
        return;
    }
    fish[3] += fish[5];
    if n == 6 {
        return;
    }
    fish[4] += fish[6];
    if n == 7 {
        return;
    }
    fish[5] += fish[7];
}
