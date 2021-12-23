use std::cmp::{max, min};
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn of_room(i: usize) -> Pod {
        match i {
            0 => Pod::A,
            1 => Pod::B,
            2 => Pod::C,
            3 => Pod::D,
            _ => unimplemented!(),
        }
    }
    fn room(self) -> usize {
        match self {
            Pod::A => 0,
            Pod::B => 1,
            Pod::C => 2,
            Pod::D => 3,
        }
    }
    fn cost(self) -> usize {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    rooms: [Vec<Pod>; 4],
    left: [Option<Pod>; 2],
    mids: [Option<Pod>; 3],
    right: [Option<Pod>; 2],
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost + self.heuristic())
            .cmp(&(other.cost + other.heuristic()))
            .reverse()
    }
}

fn show_pod(pod: Option<Pod>) -> &'static str {
    match pod {
        None => ".",
        Some(Pod::A) => "A",
        Some(Pod::B) => "B",
        Some(Pod::C) => "C",
        Some(Pod::D) => "D",
    }
}

impl State {
    fn print(&self) {
        println!("#############");
        println!(
            "#{}{}.{}.{}.{}.{}{}#",
            show_pod(self.left[0]),
            show_pod(self.left[1]),
            show_pod(self.mids[0]),
            show_pod(self.mids[1]),
            show_pod(self.mids[2]),
            show_pod(self.right[1]),
            show_pod(self.right[0]),
        );
        println!(
            "###{}#{}#{}#{}###",
            show_pod(self.rooms[0].get(1).copied()),
            show_pod(self.rooms[1].get(1).copied()),
            show_pod(self.rooms[2].get(1).copied()),
            show_pod(self.rooms[3].get(1).copied()),
        );
        println!(
            "  #{}#{}#{}#{}#  ",
            show_pod(self.rooms[0].get(0).copied()),
            show_pod(self.rooms[1].get(0).copied()),
            show_pod(self.rooms[2].get(0).copied()),
            show_pod(self.rooms[3].get(0).copied()),
        );
        println!("  #########  ");
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct StateTrace {
    trace: Vec<State>,
}
impl PartialOrd for StateTrace {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StateTrace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.trace.last()).cmp(&other.trace.last())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Location {
    Room(usize),
    Left(usize),
    Right(usize),
    Mid(usize),
}

const ALL_LOCATIONS: [Location; 11] = [
    Location::Room(0),
    Location::Room(1),
    Location::Room(2),
    Location::Room(3),
    Location::Left(0),
    Location::Left(1),
    Location::Right(0),
    Location::Right(1),
    Location::Mid(0),
    Location::Mid(1),
    Location::Mid(2),
];

const ROOM_DEPTH: usize = 4;

fn top_pod(state: &State, loc: Location) -> Option<Pod> {
    use Location::*;
    match loc {
        Room(i) => state.rooms[i].last().copied(),
        Left(i) => state.left[i],
        Right(i) => state.right[i],
        Mid(i) => state.mids[i],
    }
}

fn accepts(state: &State, loc: Location, pod: Pod) -> bool {
    use Location::*;
    match loc {
        Room(i) => i == pod.room() && state.rooms[i].iter().all(|p| *p == pod),
        Left(i) => state.left[i].is_none(),
        Right(i) => state.right[i].is_none(),
        Mid(i) => state.mids[i].is_none(),
    }
}

fn exterior_distance(start: Location, end: Location) -> usize {
    use Location::*;
    match (start, end) {
        (Room(i), Room(j)) => {
            let (l, r) = (min(i, j), max(i, j));
            2 * (r - l)
        }
        (Room(i), Mid(j)) | (Mid(j), Room(i)) => {
            if i <= j {
                // Going Right
                1 + 2 * (j - i)
            } else {
                // Going Left
                2 * (i - j) - 1
            }
        }
        (Room(i), Left(j)) | (Left(j), Room(i)) => 2 * i + 2 - j,
        (Room(i), Right(j)) | (Right(j), Room(i)) => 2 * (3 - i) + 2 - j,
        _ => unreachable!(),
    }
}

fn top_exit_distance(state: &State, loc: Location) -> usize {
    use Location::*;
    match loc {
        Room(i) => ROOM_DEPTH - state.rooms[i].len(),
        _ => 0,
    }
}

fn path(state: &State, start: Location, end: Location) -> Option<usize> {
    use Location::*;
    let pod = top_pod(state, start)?;
    if !accepts(state, end, pod) {
        return None;
    }
    if start == end {
        return None;
    }
    if let Room(i) = start {
        if state.rooms[i].iter().all(|pod| pod.room() == i) {
            return None;
        }
    }
    let blocked = match (start, end) {
        (Room(i), Room(j)) => {
            let (l, r) = (min(i, j), max(i, j));
            state.mids[l..r].iter().any(|slot| slot.is_some())
        }
        (Room(i), Mid(j)) | (Mid(j), Room(i)) => {
            if i <= j {
                // Going Right
                state.mids[i..j].iter().any(|slot| slot.is_some())
            } else {
                // Going Left
                state.mids[j + 1..i].iter().any(|slot| slot.is_some())
            }
        }
        (Room(i), Left(j)) | (Left(j), Room(i)) => {
            (j == 0 && state.left[1].is_some()) || state.mids[..i].iter().any(|slot| slot.is_some())
        }
        (Room(i), Right(j)) | (Right(j), Room(i)) => {
            (j == 0 && state.right[1].is_some())
                || state.mids[i..].iter().any(|slot| slot.is_some())
        }
        _ => return None,
    };
    if blocked {
        return None;
    }
    let base_distance = exterior_distance(start, end);
    let exit_distance = match start {
        Room(i) => ROOM_DEPTH + 1 - state.rooms[i].len(),
        Left(_) => 0,
        Right(_) => 0,
        Mid(_) => 0,
    };
    let entrence_distance = match end {
        Room(i) => ROOM_DEPTH - state.rooms[i].len(),
        Left(_) => 0,
        Right(_) => 0,
        Mid(_) => 0,
    };
    let distance = base_distance + exit_distance + entrence_distance;
    Some(distance)
}

impl State {
    fn pop_pod(&mut self, location: Location) -> Option<Pod> {
        use Location::*;
        match location {
            Room(i) => self.rooms[i].pop(),
            Mid(i) => self.mids[i].take(),
            Left(i) => self.left[i].take(),
            Right(i) => self.right[i].take(),
        }
    }
    fn push_pod(&mut self, location: Location, pod: Pod) {
        use Location::*;
        assert!(accepts(&*self, location, pod));
        match location {
            Room(i) => self.rooms[i].push(pod),
            Mid(i) => self.mids[i] = Some(pod),
            Left(i) => self.left[i] = Some(pod),
            Right(i) => self.right[i] = Some(pod),
        }
    }
    fn apply_move(&self, i: Location, j: Location) -> Option<Self> {
        let distance = path(self, i, j)?;
        let mut out = self.clone();
        let pod = out.pop_pod(i).unwrap();
        out.cost += distance * pod.cost();
        out.push_pod(j, pod);
        Some(out)
    }
    fn moves<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
        ALL_LOCATIONS.iter().flat_map(move |&i| {
            ALL_LOCATIONS.iter().filter_map(move |&j| {
                let child = self.apply_move(i, j)?;
                if child.cost + child.heuristic() < self.cost + self.heuristic() {
                    self.print();
                    child.print();
                    panic!("Heuristic violation");
                }
                Some(child)
            })
        })
    }
    fn is_solved(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(i, room)| room.len() == ROOM_DEPTH && room[0].room() == i && room[1].room() == i)
    }
    fn heuristic(&self) -> usize {
        let room_exit_cost: usize = self
            .rooms
            .iter()
            .enumerate()
            .flat_map(|(i, room)| {
                let expected_pod = Pod::of_room(i);
                room.iter().enumerate().map(move |(j, pod)| {
                    if *pod == expected_pod {
                        0
                    } else {
                        let exit_distance =
                            exterior_distance(Location::Room(i), Location::Room(pod.room()))
                                + ROOM_DEPTH
                                - j;
                        pod.cost() * exit_distance
                    }
                })
            })
            .sum();
        let room_entrance_cost: usize = self
            .rooms
            .iter()
            .enumerate()
            .flat_map(|(i, room)| {
                let expected_pod = Pod::of_room(i);
                (0..ROOM_DEPTH).map(move |j| {
                    let pod = room.get(j);
                    if pod == Some(&expected_pod) {
                        0
                    } else {
                        expected_pod.cost() * (ROOM_DEPTH - j)
                    }
                })
            })
            .sum();
        let left_cost: usize = self
            .left
            .iter()
            .enumerate()
            .filter_map(|(i, o_pod)| {
                let pod = (*o_pod)?;
                let distance = exterior_distance(Location::Left(i), Location::Room(pod.room()));
                Some(distance * pod.cost())
            })
            .sum();
        let right_cost: usize = self
            .right
            .iter()
            .enumerate()
            .filter_map(|(i, o_pod)| {
                let pod = (*o_pod)?;
                let distance = exterior_distance(Location::Right(i), Location::Room(pod.room()));
                Some(distance * pod.cost())
            })
            .sum();
        let mid_cost: usize = self
            .mids
            .iter()
            .enumerate()
            .filter_map(|(i, o_pod)| {
                let pod = (*o_pod)?;
                let distance = exterior_distance(Location::Mid(i), Location::Room(pod.room()));
                Some(distance * pod.cost())
            })
            .sum();
        room_exit_cost + room_entrance_cost + left_cost + right_cost + mid_cost
    }
}

pub fn solve1(input: &[String]) {
    let initial_state = State {
        cost: 0,
        left: [None; 2],
        right: [None; 2],
        mids: [None; 3],
        rooms: [
            vec![Pod::B, Pod::D, Pod::D, Pod::C],
            vec![Pod::A, Pod::B, Pod::C, Pod::D],
            vec![Pod::B, Pod::A, Pod::B, Pod::D],
            vec![Pod::C, Pod::C, Pod::A, Pod::A],
        ],
    };
    /*
    let initial_state = State {
        cost: 0,
        left: [None; 2],
        right: [None; 2],
        mids: [None; 3],
        rooms: [
            vec![Pod::A, Pod::B],
            vec![Pod::D, Pod::C],
            vec![Pod::C, Pod::B],
            vec![Pod::A, Pod::D],
        ],
    };
    */
    dbg!(initial_state.heuristic());
    let do_trace = false;
    if do_trace {
        let mut heap = BinaryHeap::new();
        heap.push(StateTrace {
            trace: vec![initial_state],
        });
        let cost = loop {
            let trace = heap.pop().unwrap();
            let state = trace.trace.last().unwrap();
            if state.is_solved() {
                for ss in trace.trace.windows(2) {
                    println!("Cost:{}", ss[1].cost - ss[0].cost);
                    ss[1].print();
                }
                break state.cost;
            }
            let min_cost = state.cost + state.heuristic();
            dbg!(min_cost, heap.len());
            for child in state.moves() {
                let mut new_trace = trace.clone();
                new_trace.trace.push(child);
                heap.push(new_trace);
            }
        };
        dbg!(cost);
    } else {
        let mut heap = BinaryHeap::new();
        heap.push(initial_state);
        let cost = loop {
            let state = heap.pop().unwrap();
            if state.is_solved() {
                break state.cost;
            }
            for child in state.moves() {
                heap.push(child);
            }
        };
        dbg!(cost);
    }
}

#[test]
fn test_block() {
    /*
    #############
    #...B...D.D.#
    ###B#.#C#.###
      #A#.#C#A#
      #########
    Cost:5
    #############
    #...B.A.D.D.#
    ###B#.#C#.###
      #A#.#C#.#
      #########
    */
    let initial_state = State {
        cost: 0,
        left: [None; 2],
        right: [None, Some(Pod::D)],
        mids: [Some(Pod::B), None, Some(Pod::D)],
        rooms: [
            vec![Pod::A, Pod::B],
            vec![],
            vec![Pod::C, Pod::C],
            vec![Pod::A],
        ],
    };
    assert_eq!(
        path(&initial_state, Location::Room(3), Location::Mid(1)),
        None
    );
}

#[test]
fn test_known_solution() {
    let mut state = State {
        cost: 0,
        left: [None; 2],
        right: [None; 2],
        mids: [None; 3],
        rooms: [
            vec![Pod::A, Pod::B],
            vec![Pod::D, Pod::C],
            vec![Pod::C, Pod::B],
            vec![Pod::A, Pod::D],
        ],
    };
    let moves = [
        (Location::Room(2), Location::Mid(0)),
        (Location::Room(1), Location::Room(2)),
        (Location::Room(1), Location::Mid(1)),
        (Location::Mid(0), Location::Room(1)),
        (Location::Room(0), Location::Room(1)),
        (Location::Room(3), Location::Mid(2)),
        (Location::Room(3), Location::Right(1)),
        (Location::Mid(2), Location::Room(3)),
        (Location::Mid(1), Location::Room(3)),
        (Location::Right(1), Location::Room(0)),
    ];
    for (i, j) in moves {
        match state.apply_move(i, j) {
            Some(new_state) => state = new_state,
            None => {
                state.print();
                dbg!(i, j);
                panic!("Move failed.");
            }
        }
    }
    assert!(state.is_solved());
    assert_eq!(state.cost, 12521);
}

#[test]
fn test_heuristic() {
    /*
    #############
    #...........#
    ###C#D#D#A###
      #B#A#B#C#
      #########
    #############
    #...C.......#
    ###.#D#D#A###
      #B#A#B#C#
      #########
    */
    let initial_state = State {
        cost: 0,
        left: [None; 2],
        right: [None; 2],
        mids: [None; 3],
        rooms: [
            vec![Pod::B, Pod::C],
            vec![Pod::A, Pod::D],
            vec![Pod::B, Pod::D],
            vec![Pod::C, Pod::A],
        ],
    };
    assert_eq!(initial_state.cost + initial_state.heuristic(), 12324);
    let state = State {
        cost: 200,
        left: [None; 2],
        right: [None; 2],
        mids: [Some(Pod::C), None, None],
        rooms: [
            vec![Pod::B],
            vec![Pod::A, Pod::D],
            vec![Pod::B, Pod::D],
            vec![Pod::C, Pod::A],
        ],
    };
    assert_eq!(state.cost + state.heuristic(), 12324);
}
