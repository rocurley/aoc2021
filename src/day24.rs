pub fn solve1(input: &[String]) {
    let input_nums: Vec<i64> = input[..14].iter().map(|s| s.parse().unwrap()).collect();
    let instructions: Vec<Instruction> = input[14..].iter().map(parse_instruction).collect();
    let mut z = Vec::new();
    for i in 0..14 {
        let x = z.last().copied().unwrap_or(0) + B[i];
        if A[i] {
            // true: div z 26, false: div z 1
            z.pop();
        }
        let w = input_nums[i];
        if x != w {
            z.push(w + C[i]);
        }
        println!("{}: x={}, z={:?}", i, x, &z);
    }
    let z_n: i64 = z
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, zi)| 26i64.pow(i as u32) * zi)
        .sum();
    let z2 = evaluate(input_nums, instructions);
    assert_eq!(z_n, z2);
}

pub fn solve2(input: &[String]) {
    let solutions = valid_codes(0).filter(|sol| sol.goal.is_empty());
    let out = solutions
        .map(|sol| {
            let code: i64 = sol
                .rev_input
                .into_iter()
                .enumerate()
                .map(|(i, d)| 10i64.pow(i as u32) * d)
                .sum();
            code
        })
        .min()
        .unwrap();
    dbg!(out);
}

#[derive(Debug, Clone)]
struct SearchState {
    rev_input: Vec<i64>,
    goal: Vec<i64>,
}

fn valid_codes(i: usize) -> Box<dyn Iterator<Item = SearchState>> {
    if i == 13 {
        Box::new(
            require_next(13, Vec::new())
                .into_iter()
                .map(|(input, goal)| SearchState {
                    rev_input: vec![input],
                    goal,
                }),
        )
    } else {
        Box::new(valid_codes(i + 1).flat_map(move |state| {
            require_next(i, state.goal)
                .into_iter()
                .map(move |(input, goal)| {
                    let mut rev_input = state.rev_input.clone();
                    rev_input.push(input);
                    SearchState { rev_input, goal }
                })
        }))
    }
}

struct EvalState {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl EvalState {
    fn eval_value(&self, v: Value) -> i64 {
        match v {
            Value::Const(x) => x,
            Value::Var(Variable::W) => self.w,
            Value::Var(Variable::X) => self.x,
            Value::Var(Variable::Y) => self.y,
            Value::Var(Variable::Z) => self.z,
        }
    }
    fn var_mut(&mut self, v: Variable) -> &mut i64 {
        match v {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }
}

fn require_next(i: usize, mut goal: Vec<i64>) -> Vec<(i64, Vec<i64>)> {
    if !A[i] {
        let x = goal.last().copied().unwrap_or(0) + B[i];
        return require_next_nopop(i, goal, x);
    }
    let mut out = Vec::new();
    for popped in 1..26 {
        let mut nopop_res = require_next_nopop(i, goal.clone(), popped + B[i]);
        for (_, v) in nopop_res.iter_mut() {
            v.push(popped);
        }
        out.extend(nopop_res);
    }
    out
}

fn require_next_nopop(i: usize, mut goal: Vec<i64>, x: i64) -> Vec<(i64, Vec<i64>)> {
    let mut out = Vec::new();
    // no-push:
    if 0 < x && x < 10 {
        out.push((x, goal.clone()));
    }
    // push:
    if let Some(last) = goal.pop() {
        let w = last - C[i];
        if 0 < w && w < 10 {
            out.push((w, goal));
        }
    }
    out
}

fn evaluate(input: Vec<i64>, instructions: Vec<Instruction>) -> i64 {
    let mut input = input.into_iter();
    let mut state = EvalState {
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };
    for ins in instructions {
        match ins {
            Instruction::Add(l, r) => {
                let rhs = state.eval_value(r);
                *state.var_mut(l) += rhs;
            }
            Instruction::Mul(l, r) => {
                let rhs = state.eval_value(r);
                *state.var_mut(l) *= rhs;
            }
            Instruction::Div(l, r) => {
                let rhs = state.eval_value(r);
                *state.var_mut(l) /= rhs;
            }
            Instruction::Mod(l, r) => {
                let rhs = state.eval_value(r);
                *state.var_mut(l) %= rhs;
            }
            Instruction::Eql(l, r) => {
                let rhs = state.eval_value(r);
                let lhs = state.var_mut(l);
                if *lhs == rhs {
                    *lhs = 1;
                } else {
                    *lhs = 0;
                }
            }
            Instruction::Inp(l) => {
                *state.var_mut(l) = input.next().unwrap();
            }
        }
    }
    state.z
}

fn parse_instruction(line: &String) -> Instruction {
    let mut split = line.split(" ");
    let ins_str = split.next().unwrap();
    let arg1_str = split.next().unwrap();
    let arg2_str = split.next();
    let arg1 = match arg1_str {
        "w" => Variable::W,
        "x" => Variable::X,
        "y" => Variable::Y,
        "z" => Variable::Z,
        _ => unreachable!(),
    };
    let arg2 = match arg2_str {
        Some("w") => Value::Var(Variable::W),
        Some("x") => Value::Var(Variable::X),
        Some("y") => Value::Var(Variable::Y),
        Some("z") => Value::Var(Variable::Z),
        Some(n) => Value::Const(n.parse().unwrap()),
        None => Value::Const(0),
    };
    match ins_str {
        "inp" => Instruction::Inp(arg1),
        "add" => Instruction::Add(arg1, arg2),
        "mul" => Instruction::Mul(arg1, arg2),
        "div" => Instruction::Div(arg1, arg2),
        "mod" => Instruction::Mod(arg1, arg2),
        "eql" => Instruction::Eql(arg1, arg2),
        _ => unreachable!(),
    }
}

const A: [bool; 14] = [
    false, false, false, true, false, true, true, false, false, false, true, true, true, true,
];
const B: [i64; 14] = [14, 15, 13, -10, 14, -3, -14, 12, 14, 12, -6, -6, -2, -9];
const C: [i64; 14] = [8, 11, 2, 11, 1, 5, 10, 6, 1, 11, 9, 14, 11, 2];

enum Variable {
    W,
    X,
    Y,
    Z,
}

enum Value {
    Const(i64),
    Var(Variable),
}

enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
}

/*
pub fn solve2(input: &[String]) {
    let bitvectors = (0..1 << 12).rev().map(|x| {
        format!("{:012b}", x)
            .chars()
            .map(|b| b == '1')
            .collect::<Vec<bool>>()
    });
    'search: for bv in bitvectors {
        let mut z = Vec::new();
        let mut input = Vec::new();
        for i in 0..12 {
            let x = z.last().copied().unwrap_or(0) + B[i];
            if A[i] {
                // true: div z 26, false: div z 1
                z.pop();
            }
            if bv[i] {
                input.push(9);
            } else {
                if 0 < x && x < 10 {
                    input.push(x);
                } else {
                    continue 'search;
                }
            }
            let w = input[i];
            if x != w {
                z.push(w + C[i]);
            }
        }
        if z.iter().all(|zi| *zi == 0) {
            println!("{:?}", input);
            return;
        }
    }
    println!("Search failed!")
}
*/
