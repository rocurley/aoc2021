use std::collections::HashMap;
use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let scans = input
        .split(|line| line == "")
        .map(|scan_lines| {
            scan_lines[1..]
                .iter()
                .map(|line| {
                    line.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<i32>>()
                        .as_slice()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[i32; 3]>>()
        })
        .collect::<Vec<Vec<[i32; 3]>>>();
    let links: Vec<(usize, usize, TF)> = scans
        .iter()
        .enumerate()
        .flat_map(|(i, xs)| {
            scans[i + 1..]
                .iter()
                .enumerate()
                .filter_map(move |(j_rel, ys)| {
                    let j = i + j_rel + 1;
                    let tf = orientations()
                        .filter_map(|o| {
                            matching_deltas(
                                point_deltas(xs).collect(),
                                point_deltas(ys).collect(),
                                o,
                            )
                        })
                        .next()?;
                    Some((i, j, tf))
                })
        })
        .collect();
    let sensor_transforms = flatten_links(links);
    for (i, tf) in sensor_transforms.iter() {
        let pt = tf.translation;
        println!("{}: [{},{},{}]", i, pt[0], pt[1], pt[2]);
    }
    let merged: HashSet<[i32; 3]> = scans
        .iter()
        .enumerate()
        .flat_map(|(i, scan)| {
            let tf = sensor_transforms[&i];
            scan.iter().map(move |pt| tf.apply(*pt))
        })
        .collect();
    dbg!(merged.len());
    let size = sensor_transforms
        .values()
        .flat_map(|tf1| {
            sensor_transforms.values().map(|tf2| {
                let delta = pt_sub(tf1.translation, tf2.translation);
                delta[0].abs() + delta[1].abs() + delta[2].abs()
            })
        })
        .max()
        .unwrap();
    dbg!(size);
}
fn flatten_links(mut links: Vec<(usize, usize, TF)>) -> HashMap<usize, TF> {
    let mut out = HashMap::new();
    out.insert(0, TF_O);
    while !links.is_empty() {
        for (i, (dest, origin, tf)) in links.iter().enumerate() {
            if let Some(dest_tf) = out.get(&dest).copied() {
                out.insert(*origin, dest_tf.of(&tf));
                links.remove(i);
                break;
            }
            if let Some(dest_tf) = out.get(&origin).copied() {
                let (dest, origin, tf) = (origin, dest, tf.inv());
                out.insert(*origin, dest_tf.of(&tf));
                links.remove(i);
                break;
            }
        }
    }
    out
}

fn find_final(mut i: usize, merges: &HashMap<usize, usize>) -> usize {
    loop {
        match merges.get(&i) {
            Some(j) => i = *j,
            None => return i,
        }
    }
}

fn merge(mut scans: Vec<Vec<[i32; 3]>>, mut links: Vec<(usize, usize, TF)>) -> Vec<[i32; 3]> {
    let mut merges: HashMap<usize, usize> = HashMap::new();
    while scans.len() > 1 {
        let source = scans.pop().unwrap();
        let (target_i, source_i, tf) = links.pop().unwrap();
        let target_i = find_final(target_i, &merges);
        let source_i = find_final(source_i, &merges);
        let target = &mut scans[target_i];
        for pt in source {
            target.push(tf.apply(pt));
        }
        merges.insert(source_i, target_i);
    }
    scans.pop().unwrap()
}

fn matching_deltas(
    xs: HashMap<[i32; 3], [i32; 3]>,
    ys: HashMap<[i32; 3], [i32; 3]>,
    o: Orientation,
) -> Option<TF> {
    let mut matches = ys.iter().filter_map(|(dy, y)| {
        let x = xs.get(&o.apply(*dy))?;
        let tf = TF {
            orientation: o,
            translation: pt_sub(*x, o.apply(*y)),
        };
        assert_eq!(*x, tf.apply(*y));
        Some(tf)
    });
    let tf = matches.next()?;
    let c = matches
        .filter(|tf2| tf.translation == tf2.translation)
        .count()
        + 1;
    if c >= 12 * 11 {
        Some(tf)
    } else {
        None
    }
}

fn point_deltas<'a>(points: &'a [[i32; 3]]) -> impl Iterator<Item = ([i32; 3], [i32; 3])> + 'a {
    points.iter().copied().enumerate().flat_map(|(i, x)| {
        points
            .iter()
            .copied()
            .enumerate()
            .filter_map(move |(j, y)| {
                if i == j {
                    None
                } else {
                    Some((pt_sub(x, y), x))
                }
            })
    })
}

fn pt_sub(x: [i32; 3], y: [i32; 3]) -> [i32; 3] {
    [x[0] - y[0], x[1] - y[1], x[2] - y[2]]
}
fn pt_add(x: [i32; 3], y: [i32; 3]) -> [i32; 3] {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2]]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Orientation {
    permutation: [usize; 3],
    signs: [i32; 3],
}

const ID_O: Orientation = Orientation {
    permutation: [1, 2, 3],
    signs: [1, 1, 1],
};

const TF_O: TF = TF {
    orientation: ID_O,
    translation: [0, 0, 0],
};

impl Orientation {
    fn apply(&self, pt: [i32; 3]) -> [i32; 3] {
        let p = self.permutation;
        let signs = self.signs;
        [
            pt[p[0] - 1] * signs[0],
            pt[p[1] - 1] * signs[1],
            pt[p[2] - 1] * signs[2],
        ]
    }
    fn of(&self, other: &Self) -> Self {
        let permutation = [
            other.permutation[self.permutation[0] - 1],
            other.permutation[self.permutation[1] - 1],
            other.permutation[self.permutation[2] - 1],
        ];
        let signs = [
            self.signs[0] * other.signs[self.permutation[0] - 1],
            self.signs[1] * other.signs[self.permutation[1] - 1],
            self.signs[2] * other.signs[self.permutation[2] - 1],
        ];
        Orientation { permutation, signs }
    }
    fn inv(self) -> Self {
        let permutation = [
            self.permutation.into_iter().position(|x| x == 1).unwrap() + 1,
            self.permutation.into_iter().position(|x| x == 2).unwrap() + 1,
            self.permutation.into_iter().position(|x| x == 3).unwrap() + 1,
        ];
        let signs = [
            self.signs[permutation[0] - 1],
            self.signs[permutation[1] - 1],
            self.signs[permutation[2] - 1],
        ];
        Orientation { permutation, signs }
    }
}

#[test]
fn test_compose() {
    let pt = [1, 2, 3];
    for o1 in orientations() {
        for o2 in orientations() {
            assert_eq!(o1.of(&o2).apply(pt), o1.apply(o2.apply(pt)));
        }
    }
}

#[test]
fn test_inv() {
    for o1 in orientations() {
        let id = o1.of(&o1.inv());
        assert_eq!(id, ID_O);
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TF {
    orientation: Orientation,
    translation: [i32; 3],
}

impl TF {
    fn apply(&self, pt: [i32; 3]) -> [i32; 3] {
        pt_add(self.translation, self.orientation.apply(pt))
    }
    fn of(&self, other: &Self) -> Self {
        let orientation = self.orientation.of(&other.orientation);
        let translation = pt_add(self.translation, self.orientation.apply(other.translation));
        TF {
            orientation,
            translation,
        }
    }
    fn inv(self) -> Self {
        let orientation = self.orientation.inv();
        let ti = orientation.apply(self.translation);
        let translation = [-ti[0], -ti[1], -ti[2]];
        TF {
            orientation,
            translation,
        }
    }
}
#[test]
fn test_compose_tf() {
    let pt = [1, 2, 3];
    for o1 in orientations() {
        let tf1 = TF {
            orientation: o1,
            translation: [4, 5, 6],
        };
        for o2 in orientations() {
            let tf2 = TF {
                orientation: o2,
                translation: [7, 8, 9],
            };
            assert_eq!(tf1.of(&tf2).apply(pt), tf1.apply(tf2.apply(pt)));
        }
    }
}
#[test]
fn test_inv_tf() {
    let pt = [1, 2, 3];
    for o1 in orientations() {
        let tf1 = TF {
            orientation: o1,
            translation: [4, 5, 6],
        };
        assert_eq!(tf1.of(&tf1.inv()), TF_O);
    }
}

fn orientations() -> impl Iterator<Item = Orientation> {
    [
        ([1, 2, 3], 1),
        ([1, 3, 2], -1),
        ([2, 1, 3], -1),
        ([2, 3, 1], 1),
        ([3, 1, 2], 1),
        ([3, 2, 1], -1),
    ]
    .into_iter()
    .flat_map(move |(permutation, sign)| {
        [
            [1, 1, 1],
            [1, 1, -1],
            [1, -1, 1],
            [1, -1, -1],
            [-1, 1, 1],
            [-1, 1, -1],
            [-1, -1, 1],
            [-1, -1, -1],
        ]
        .into_iter()
        .filter(move |signs| sign * signs[0] * signs[1] * signs[2] == 1)
        .map(move |signs| Orientation { permutation, signs })
    })
}
