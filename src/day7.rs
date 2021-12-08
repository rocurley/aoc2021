pub fn solve1(input: &[String]) {
    let mut positions: Vec<i32> = input[0].split(",").map(|x| x.parse().unwrap()).collect();
    positions.sort();
    let median = positions[positions.len() / 2];
    let fuel: i32 = positions.iter().copied().map(|x| (x - median).abs()).sum();
    println!("{}", fuel);
    let mean: f64 = (positions.iter().sum::<i32>() as f64) / (positions.len() as f64);
    let mean = mean.round() as i32;
    for mean in (mean - 1..=mean + 1) {
        let fuel: i32 = positions
            .iter()
            .copied()
            .map(|x| ((x - mean).abs() + 1) * (x - mean).abs() / 2)
            .sum();
        println!("{}", fuel);
    }
}
