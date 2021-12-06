use std::collections::HashMap;

use recap::Recap;
use serde::Deserialize;
use std::cmp::{max, min};

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<startx>\d+),(?P<starty>\d+) -> (?P<endx>\d+),(?P<endy>\d+)"#)]
struct Vent {
    startx: i32,
    starty: i32,
    endx: i32,
    endy: i32,
}

pub fn solve1(input: &[String]) {
    let vents = input.iter().map(|line| line.parse::<Vent>().unwrap());
    let mut counts: HashMap<(i32, i32), u32> = HashMap::new();
    for vent in vents {
        if vent.startx == vent.endx {
            for y in min(vent.starty, vent.endy)..=max(vent.starty, vent.endy) {
                *counts.entry((vent.startx, y)).or_insert(0) += 1;
            }
        } else if vent.starty == vent.endy {
            for x in min(vent.startx, vent.endx)..=max(vent.startx, vent.endx) {
                *counts.entry((x, vent.starty)).or_insert(0) += 1;
            }
        } else {
            let ys = min(vent.starty, vent.endy)..=max(vent.starty, vent.endy);
            let xs = min(vent.startx, vent.endx)..=max(vent.startx, vent.endx);
            if (vent.starty < vent.endy) != (vent.startx < vent.endx) {
                for (x, y) in xs.rev().zip(ys) {
                    *counts.entry((x, y)).or_insert(0) += 1;
                }
            } else {
                for (x, y) in xs.zip(ys) {
                    *counts.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
    }
    let count = counts.iter().filter(|(_k, &v)| v >= 2).count();
    println!("{}", count);
}
