pub fn solve1(input: &[String]) {
    let mut nums = input.iter().map(|line| read_snum(&mut line.as_str()));
    let nums_0 = nums.next().unwrap();
    let res = nums.fold(nums_0, |l, r| {
        let mut out = SNum::Pair(Box::new(l), Box::new(r));
        reduce(&mut out);
        out
    });
    dbg!(res.magnitude());
}

pub fn solve2(input: &[String]) {
    let nums: Vec<SNum> = input
        .iter()
        .map(|line| read_snum(&mut line.as_str()))
        .collect();
    let out = nums
        .iter()
        .flat_map(|l| {
            nums.iter().map(|r| {
                let mut out = SNum::Pair(Box::new(l.clone()), Box::new(r.clone()));
                reduce(&mut out);
                out.magnitude()
            })
        })
        .max();
    dbg!(out);
}

fn read_1<'a>(s: &mut &'a str) -> &'a str {
    let (first, rest) = s.split_at(1);
    *s = rest;
    first
}

fn read_snum(s: &mut &str) -> SNum {
    let (first, rest) = s.split_at(1);
    if first == "[" {
        *s = rest;
        let l = read_snum(s);
        let r = read_snum(s);
        if let Some((_, rest)) = s.split_once(&[',', ']'][..]) {
            *s = rest;
        }
        SNum::Pair(Box::new(l), Box::new(r))
    } else {
        let (n, rest) = s.split_once(&[',', ']'][..]).unwrap();
        *s = rest;
        SNum::Num(n.parse().unwrap())
    }
}

#[derive(Clone)]
enum SNum {
    Num(i32),
    Pair(Box<SNum>, Box<SNum>),
}

impl std::fmt::Debug for SNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SNum::Num(x) => f.write_fmt(format_args!("{}", x)),
            SNum::Pair(l, r) => f.debug_tuple("").field(l).field(r).finish(),
        }
    }
}

impl SNum {
    fn magnitude(&self) -> i32 {
        match self {
            SNum::Num(x) => *x,
            SNum::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn leftmost_num(s: &mut SNum) -> &mut i32 {
    match s {
        SNum::Num(x) => x,
        SNum::Pair(l, _r) => leftmost_num(l),
    }
}

fn rightmost_num(s: &mut SNum) -> &mut i32 {
    match s {
        SNum::Num(x) => x,
        SNum::Pair(_l, r) => rightmost_num(r),
    }
}

fn explode(s: &mut SNum, depth: usize) -> (bool, Option<i32>, Option<i32>) {
    match s {
        &mut SNum::Num(_) => (false, None, None),
        SNum::Pair(l, r) => {
            if depth == 4 {
                let mut old = SNum::Num(0);
                std::mem::swap(&mut old, s);
                if let SNum::Pair(bl, br) = old {
                    if let (SNum::Num(l), SNum::Num(r)) = (*bl, *br) {
                        return (true, Some(l), Some(r));
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            } else {
                let (l_exploded, left_exploder, right_exploder) = explode(l, depth + 1);
                if l_exploded {
                    if let Some(re) = right_exploder {
                        *leftmost_num(r) += re;
                    }
                    return (true, left_exploder, None);
                }
                let (r_exploded, left_exploder, right_exploder) = explode(r, depth + 1);
                if r_exploded {
                    if let Some(le) = left_exploder {
                        *rightmost_num(l) += le;
                    }
                    return (true, None, right_exploder);
                }
                return (false, None, None);
            }
        }
    }
}
fn split(s: &mut SNum) -> bool {
    match s {
        SNum::Num(x) if *x >= 10 => {
            *s = SNum::Pair(
                Box::new(SNum::Num(*x / 2)),
                Box::new(SNum::Num((*x + 1) / 2)),
            );
            true
        }
        SNum::Num(_) => false,
        SNum::Pair(l, r) => split(l) || split(r),
    }
}

fn reduce(s: &mut SNum) {
    loop {
        if explode(s, 0).0 {
            continue;
        }
        if split(s) {
            continue;
        }
        break;
    }
}
