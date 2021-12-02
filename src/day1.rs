pub fn solve1(input: &[String]) {
    let mut last = "";
    let mut out = 0;
    for x in input {
        if x.as_str() > last {
            out += 1;
        }
        last = &x;
    }
    println!("{}", out);
}
pub fn solve2(input: &[String]) {
    let mut last = 0;
    let mut out = -1;
    for window in input.windows(3) {
        let sum: i64 = window.iter().map(|x| x.parse::<i64>().unwrap()).sum();
        if sum > last {
            out += 1;
        }
        last = sum;
    }
    println!("{}", out);
}
