use std::collections::HashMap;
use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let enhancement: Vec<bool> = input[0].chars().map(|c| c == '#').collect();

    let mut image = read_image(&input[2..]);
    for _ in 0..50 {
        image = step_image(image, &enhancement);
    }
    //render(&image);
    assert!(!image.bg);
    let count = image.pixels.values().filter(|p| **p).count();
    dbg!(count);
    /*
    let expected_strings_2 = [
        "...............".to_owned(),
        "...............".to_owned(),
        "...............".to_owned(),
        "..........#....".to_owned(),
        "....#..#.#.....".to_owned(),
        "...#.#...###...".to_owned(),
        "...#...##.#....".to_owned(),
        "...#.....#.#...".to_owned(),
        "....#.#####....".to_owned(),
        ".....#.#####...".to_owned(),
        "......##.##....".to_owned(),
        ".......###.....".to_owned(),
        "...............".to_owned(),
        "...............".to_owned(),
        "...............".to_owned(),
    ];
    let expected = read_image(&expected_strings_2);
    assert_eq!(image, expected);
    */
}

fn neighbors((i, j): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    (-1..=1).flat_map(move |di| (-1..=1).map(move |dj| (i + di, j + dj)))
}

struct Image {
    bg: bool,
    pixels: HashMap<(i32, i32), bool>,
}

fn enhancement_index(pt: (i32, i32), image: &Image) -> usize {
    let bits =
        neighbors(pt).map(|neighbor| image.pixels.get(&neighbor).copied().unwrap_or(image.bg));
    let bitstring: String = bits.map(|b| if b { '1' } else { '0' }).collect();
    usize::from_str_radix(&bitstring, 2).unwrap()
}

fn step_image(image: Image, enhancement: &[bool]) -> Image {
    let new_bg = enhancement[if image.bg { 0b111_111_111 } else { 0 }];
    let candidates: HashSet<(i32, i32)> =
        image.pixels.keys().flat_map(|pt| neighbors(*pt)).collect();
    let pixels = candidates
        .into_iter()
        .map(|pt| {
            let idx = enhancement_index(pt, &image);
            (pt, enhancement[idx])
        })
        .collect();
    Image { pixels, bg: new_bg }
}

fn read_image(input: &[String]) -> Image {
    let mut pixels = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, px) in row.chars().enumerate() {
            if px == '#' {
                pixels.insert((i as i32, j as i32), true);
            }
        }
    }
    Image { pixels, bg: false }
}

fn render(image: &HashSet<(i32, i32)>) {
    let min_i = image.iter().copied().map(|(i, _)| i).min().unwrap();
    let max_i = image.iter().copied().map(|(i, _)| i).max().unwrap();
    let min_j = image.iter().copied().map(|(j, _)| j).min().unwrap();
    let max_j = image.iter().copied().map(|(j, _)| j).max().unwrap();
    for i in min_i..=max_i {
        let s: String = (min_j..=max_j)
            .map(|j| if image.contains(&(i, j)) { '#' } else { '.' })
            .collect();
        println!("{}", s);
    }
}
