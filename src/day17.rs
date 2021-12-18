use std::collections::HashMap;
use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let ymin = -142;
    let ymax = -88;
    let xmin = 128;
    let xmax = 160;
    /*
    let ymin = -10;
    let ymax = -5;
    let xmin = 20;
    let xmax = 30;
    */
    let valid_vy: Vec<(usize, i32)> = (ymin..=-ymin)
        .flat_map(|vy| {
            evolve_y(vy)
                .enumerate()
                .take_while(|(_t, y)| *y >= ymin)
                .filter(|(_t, y)| *y <= ymax)
                .map(move |(t, _y)| (t, vy))
        })
        .collect();
    let valid_vx: Vec<(usize, i32)> = (0..=xmax)
        .flat_map(|vx| {
            evolve_x(vx)
                .enumerate()
                .take_while(|(_t, x)| *x <= xmax)
                .filter(|(_t, x)| *x >= xmin)
                .map(move |(t, _x)| (t, vx))
        })
        .collect();
    dbg!(&valid_vy);
    dbg!(&valid_vx);
    let mut vy_by_t: HashMap<usize, HashSet<i32>> = HashMap::new();
    for (t, vy) in valid_vy {
        vy_by_t
            .entry(t)
            .or_insert_with(|| HashSet::new())
            .insert(vy);
    }
    let max_t = *vy_by_t.keys().max().unwrap();
    let mut vx_vy: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (t, vx) in valid_vx {
        let vys = vx_vy.entry(vx).or_insert_with(HashSet::new);
        if t as i32 == vx {
            for t in (t..=max_t) {
                if let Some(new_vys) = vy_by_t.get(&t) {
                    vys.extend(new_vys);
                }
            }
        } else {
            if let Some(new_vys) = vy_by_t.get(&t) {
                vys.extend(new_vys);
            }
        }
    }
    dbg!(&vx_vy);
    let out: usize = vx_vy.values().map(|set| set.len()).sum();
    dbg!(out);
}

fn evolve_y(mut vy: i32) -> impl Iterator<Item = i32> {
    let mut y = 0;
    (0..).map(move |_| {
        y += vy;
        vy -= 1;
        y
    })
}

fn evolve_x(vx: i32) -> impl Iterator<Item = i32> {
    let mut x = 0;
    (0..=vx).rev().map(move |vx| {
        x += vx;
        x
    })
}
