pub fn solve1(input: &[String]) {
    let mut x = 0;
    let mut y = 0;
    for row in input {
        let (direction, n_str) = row.split_once(" ").unwrap();
        let n: i32 = n_str.parse().unwrap();
        match direction {
            "forward" => x += n,
            "down" => y += n,
            "up" => y -= n,
            _ => panic!("unexpected direction"),
        }
    }
    println!("{}", x * y);
}
pub fn solve2(input: &[String]) {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for row in input {
        let (direction, n_str) = row.split_once(" ").unwrap();
        let n: i32 = n_str.parse().unwrap();
        match direction {
            "forward" => {
                x += n;
                y += n * aim
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!("unexpected direction"),
        }
    }
    println!("{}", x * y);
}
