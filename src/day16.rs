pub fn solve1(input: &[String]) {
    let bits: String = input[0]
        .chars()
        .map(move |d| {
            let n = d.to_digit(16).unwrap();
            format!("{:04b}", n)
        })
        .collect::<Vec<String>>()
        .join("");
    let (packet, _) = parse_packet(&bits);
    dbg!(sum_versions(&packet));
    dbg!(evaluate(&packet));
}

fn sum_versions(p: &Packet) -> usize {
    match &p.content {
        PacketContent::Literal(_) => p.version as usize,
        PacketContent::Operator { children, .. } => {
            let children_sum: usize = children.iter().map(sum_versions).sum();
            p.version as usize + children_sum
        }
    }
}

fn evaluate(p: &Packet) -> u64 {
    match &p.content {
        PacketContent::Literal(n) => *n,
        PacketContent::Operator { ty: 0, children } => children.iter().map(evaluate).sum(),
        PacketContent::Operator { ty: 1, children } => children.iter().map(evaluate).product(),
        PacketContent::Operator { ty: 2, children } => children.iter().map(evaluate).min().unwrap(),
        PacketContent::Operator { ty: 3, children } => children.iter().map(evaluate).max().unwrap(),
        PacketContent::Operator { ty, children } => {
            let x = evaluate(&children[0]);
            let y = evaluate(&children[1]);
            let cond = match ty {
                5 => x > y,
                6 => x < y,
                7 => x == y,
                _ => unimplemented!(),
            };
            if cond {
                1
            } else {
                0
            }
        }
    }
}

fn parse_packet(s: &str) -> (Packet, &str) {
    let version = u8::from_str_radix(&s[..3], 2).unwrap();
    let ty = u8::from_str_radix(&s[3..6], 2).unwrap();
    if ty == 4 {
        let mut n = 0;
        let mut rest = &s[6..];
        loop {
            let chunk = &rest[..5];
            rest = &rest[5..];
            n *= 16;
            n += u64::from_str_radix(&chunk[1..], 2).unwrap();
            if &chunk[0..1] == "0" {
                let packet = Packet {
                    version,
                    content: PacketContent::Literal(n),
                };
                return (packet, rest);
            }
        }
    } else {
        let lt_id = &s[6..7];
        if lt_id == "0" {
            let children_len = usize::from_str_radix(&s[7..22], 2).unwrap();
            let mut children_str = &s[22..22 + children_len];
            let mut children = Vec::new();
            while !children_str.is_empty() {
                let (new_child, new_children_str) = parse_packet(children_str);
                children.push(new_child);
                children_str = new_children_str;
            }
            (
                Packet {
                    version,
                    content: PacketContent::Operator { ty, children },
                },
                &s[22 + children_len..],
            )
        } else {
            let n_children = usize::from_str_radix(&s[7..18], 2).unwrap();
            let mut rest = &s[18..];
            let children = (0..n_children)
                .map(|_| {
                    let (child, new_rest) = parse_packet(rest);
                    rest = new_rest;
                    child
                })
                .collect();
            (
                Packet {
                    version,
                    content: PacketContent::Operator { ty, children },
                },
                rest,
            )
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    content: PacketContent,
}

#[derive(Debug)]
enum PacketContent {
    Literal(u64),
    Operator { ty: u8, children: Vec<Packet> },
}
