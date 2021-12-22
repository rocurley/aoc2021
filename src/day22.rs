use std::cmp::max;
use std::cmp::min;
use std::ops::RangeInclusive;
pub fn solve1(input: &[String]) {
    let commands: Vec<Command> = input
        .iter()
        .map(|line| {
            let (verb, rest) = line.split_once(" ").unwrap();
            let [xpart, ypart, zpart]: [&str; 3] = rest
                .split(",")
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap();
            let (xmin, xmax) = xpart.strip_prefix("x=").unwrap().split_once("..").unwrap();
            let (ymin, ymax) = ypart.strip_prefix("y=").unwrap().split_once("..").unwrap();
            let (zmin, zmax) = zpart.strip_prefix("z=").unwrap().split_once("..").unwrap();
            Command {
                verb: verb == "on",
                xrange: xmin.parse().unwrap()..=xmax.parse().unwrap(),
                yrange: ymin.parse().unwrap()..=ymax.parse().unwrap(),
                zrange: zmin.parse().unwrap()..=zmax.parse().unwrap(),
            }
        })
        .collect();
    /*
    let mut out = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                let on = commands
                    .iter()
                    .rev()
                    .find(|cmd| cmd.contains([x, y, z]))
                    .map_or(false, |cmd| cmd.verb);
                if on {
                    out += 1;
                }
            }
        }
    }
    */
    let merged = commands.into_iter().fold(Vec::new(), merge);
    let out: i64 = merged.iter().map(volume).sum();
    dbg!(out);
}

fn merge(cmds: Vec<Command>, new_cmd: Command) -> Vec<Command> {
    let mut out: Vec<Command> = cmds
        .into_iter()
        .flat_map(|cmd| sub(&cmd, &new_cmd).into_iter())
        .collect();
    if new_cmd.verb {
        out.push(new_cmd);
    }
    out
}

fn volume(cmd: &Command) -> i64 {
    (*cmd.xrange.end() as i64 - *cmd.xrange.start() as i64 + 1)
        * (*cmd.yrange.end() as i64 - *cmd.yrange.start() as i64 + 1)
        * (*cmd.zrange.end() as i64 - *cmd.zrange.start() as i64 + 1)
}

fn range_overlap(l: &RangeInclusive<i32>, r: &RangeInclusive<i32>) -> bool {
    l.start() <= r.end() && r.start() <= l.end()
}

fn range_intersection(l: &RangeInclusive<i32>, r: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    *max(l.start(), r.start())..=*min(l.end(), r.end())
}

fn cubes_overlap(l: &Command, r: &Command) -> bool {
    (range_overlap(&l.xrange, &r.xrange)
        && range_overlap(&l.yrange, &r.yrange)
        && range_overlap(&l.zrange, &r.zrange))
}

fn sub(l: &Command, r: &Command) -> Vec<Command> {
    if !cubes_overlap(l, r) {
        return vec![l.clone()];
    }
    let verb = l.verb;
    let yrange = range_intersection(&l.yrange, &r.yrange);
    let zrange = range_intersection(&l.zrange, &r.zrange);
    let mut out = vec![
        Command {
            verb,
            xrange: l.xrange.clone(),
            yrange: l.yrange.clone(),
            zrange: *l.zrange.start()..=*r.zrange.start() - 1,
        },
        Command {
            verb,
            xrange: l.xrange.clone(),
            yrange: l.yrange.clone(),
            zrange: *r.zrange.end() + 1..=*l.zrange.end(),
        },
        Command {
            verb,
            xrange: l.xrange.clone(),
            yrange: *l.yrange.start()..=*r.yrange.start() - 1,
            zrange: zrange.clone(),
        },
        Command {
            verb,
            xrange: l.xrange.clone(),
            yrange: *r.yrange.end() + 1..=*l.yrange.end(),
            zrange: zrange.clone(),
        },
        Command {
            verb,
            xrange: *l.xrange.start()..=*r.xrange.start() - 1,
            yrange: yrange.clone(),
            zrange: zrange.clone(),
        },
        Command {
            verb,
            xrange: *r.xrange.end() + 1..=*l.xrange.end(),
            yrange: yrange.clone(),
            zrange: zrange.clone(),
        },
    ];
    for (i, res) in out.iter().enumerate() {
        assert!(
            !cubes_overlap(res, r),
            "out[{}] = {:?} overlaps {:?}",
            i,
            res,
            r
        );
    }
    out.retain(nonempty);
    out
}

fn nonempty(cmd: &Command) -> bool {
    !cmd.xrange.is_empty() && !cmd.yrange.is_empty() && !cmd.zrange.is_empty()
}

#[derive(Clone, Eq, Debug, PartialEq)]
struct Command {
    verb: bool,
    xrange: RangeInclusive<i32>,
    yrange: RangeInclusive<i32>,
    zrange: RangeInclusive<i32>,
}

impl Command {
    fn contains(&self, pt: [i32; 3]) -> bool {
        self.xrange.contains(&pt[0]) && self.yrange.contains(&pt[1]) && self.zrange.contains(&pt[2])
    }
}
