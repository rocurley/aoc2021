use nalgebra::DMatrix;
use nalgebra::DVector;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Sub {
    initial: [u8; 2],
    result: ([u8; 2], [u8; 2]),
}

pub fn solve1(input: &[String]) {
    let start: Vec<[u8; 2]> = input[0]
        .as_bytes()
        .windows(2)
        .map(|w| w.try_into().unwrap())
        .collect();

    let subs: Vec<Sub> = input[2..]
        .iter()
        .map(|line| {
            let (init, res) = line.split_once(" -> ").unwrap();
            let res = res.as_bytes()[0];
            let initial = init.as_bytes().try_into().unwrap();
            Sub {
                initial,
                result: ([initial[0], res], [res, initial[1]]),
            }
        })
        .collect();
    let mut alphabet = HashMap::new();
    let mut i = 0;
    for pair in start.iter() {
        if let Entry::Vacant(e) = alphabet.entry(*pair) {
            e.insert(i);
            i += 1;
        }
    }
    for sub in subs.iter() {
        if let Entry::Vacant(e) = alphabet.entry(sub.initial) {
            e.insert(i);
            i += 1;
        }
        if let Entry::Vacant(e) = alphabet.entry(sub.result.0) {
            e.insert(i);
            i += 1;
        }
        if let Entry::Vacant(e) = alphabet.entry(sub.result.1) {
            e.insert(i);
            i += 1;
        }
    }
    let rev_alphabet: HashMap<usize, [u8; 2]> = alphabet.iter().map(|(k, v)| (*v, *k)).collect();
    let mut mat = DMatrix::from_element(alphabet.len(), alphabet.len(), 0.0);
    for sub in subs {
        let j = alphabet[&sub.initial];
        let i0 = alphabet[&sub.result.0];
        let i1 = alphabet[&sub.result.1];
        dbg!(j);
        mat[(i0, j)] += 1.0;
        mat[(i1, j)] += 1.0;
    }
    let mut v0 = DVector::from_element(alphabet.len(), 0.0);
    for pair in start.iter() {
        v0[alphabet[pair]] += 1.0;
    }
    let mut mat10 = mat.clone();
    let test_mat = DMatrix::from_element(1, 1, 2.0);
    assert_eq!(&test_mat, &test_mat.pow(1).unwrap());
    for _ in 1..40 {
        mat10 *= &mat;
    }
    let v10 = mat10 * v0;
    let mut counts10 = HashMap::new();
    for (i, count) in v10.iter().enumerate() {
        let pair = rev_alphabet[&i];
        dbg!(pair[0] as char, pair[1] as char, count);
        *counts10.entry(pair[0] as char).or_insert(0) += *count as usize;
        *counts10.entry(pair[1] as char).or_insert(0) += *count as usize;
    }
    *counts10.get_mut(&(start[0][0] as char)).unwrap() += 1;
    *counts10
        .get_mut(&(start.last().unwrap()[1] as char))
        .unwrap() += 1;
    dbg!(&counts10);
    let min = counts10.values().min().unwrap() / 2;
    let max = counts10.values().max().unwrap() / 2;
    dbg!(max, min);
    dbg!((max - min));
}
