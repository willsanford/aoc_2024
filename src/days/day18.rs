use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Binary,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HeapPosition {
    pos: (i32, i32),
    cost: u64,
}

impl Ord for HeapPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for HeapPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn print_board(board: &HashSet<(i32, i32)>) {
    let n = board.iter().map(|i| i.0).max().unwrap() as usize + 1;
    let m = board.iter().map(|j| j.1).max().unwrap() as usize + 1;

    let mut board_str: Vec<Vec<char>> = vec![vec![' '; m]; n];
    for (i, j) in board {
        board_str[*i as usize][*j as usize] = 'O';
    }
    for line in board_str {
        println!("{}", line.iter().collect::<String>());
    }
}
pub fn part1(input: String) -> u64 {
    let (bytes, board_size) = if input.lines().count() > 100 {
        (1024, 70)
    } else {
        (12, 6)
    };
    let n = board_size;
    let coords: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let (i, j) = line.split_once(',').unwrap();
            (j.parse::<i32>().unwrap(), i.parse::<i32>().unwrap())
        })
        .collect();

    let mut board: HashSet<(i32, i32)> =
        (0..=n).flat_map(|i| (0..=n).map(move |j| (i, j))).collect();
    for pos in coords.iter().take(bytes) {
        board.remove(&pos);
    }

    let mut distances: HashMap<(i32, i32), u64> = HashMap::new();
    distances.insert((0, 0), 0);
    let mut to_check: BinaryHeap<HeapPosition> = BinaryHeap::new();
    to_check.push(HeapPosition {
        pos: (0, 0),
        cost: 0,
    });
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some(hp) = to_check.pop() {
        let ((i, j), c) = (hp.pos, hp.cost);
        for (di, dj) in &dirs {
            let next_p = (i + di, j + dj);
            if match (board.contains(&next_p), distances.get(&next_p)) {
                (true, None) => true,
                (true, Some(c_)) if c + 1 < *c_ => true,
                _ => false,
            } {
                distances.insert(next_p, c + 1);
                to_check.push(HeapPosition {
                    pos: next_p,
                    cost: c + 1,
                });
            }
        }
    }

    *distances.get(&(n, n)).unwrap()
}

pub fn part2(input: String) -> u64 {
    let (bytes, board_size) = if input.lines().count() > 100 {
        (1024, 70)
    } else {
        (12, 6)
    };
    let n = board_size;
    let coords: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let (i, j) = line.split_once(',').unwrap();
            (j.parse::<i32>().unwrap(), i.parse::<i32>().unwrap())
        })
        .collect();

    let mut board: HashSet<(i32, i32)> =
        (0..=n).flat_map(|i| (0..=n).map(move |j| (i, j))).collect();

    let (first_bytes, second_bytes) = coords.split_at(bytes);
    for pos in first_bytes {
        board.remove(&pos);
    }
    let a: &(i32, i32) = second_bytes
        .iter()
        .find_map(|to_remove| {
            board.remove(to_remove);
            let mut distances: HashMap<(i32, i32), u64> = HashMap::new();
            distances.insert((0, 0), 0);
            let mut to_check: BinaryHeap<HeapPosition> = BinaryHeap::new();
            to_check.push(HeapPosition {
                pos: (0, 0),
                cost: 0,
            });
            let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            while let Some(hp) = to_check.pop() {
                let ((i, j), c) = (hp.pos, hp.cost);
                for (di, dj) in &dirs {
                    let next_p: (i32, i32) = (i + di, j + dj);
                    if match (board.contains(&next_p), distances.get(&next_p)) {
                        (true, None) => true,
                        (true, Some(c_)) if c + 1 < *c_ => true,
                        _ => false,
                    } {
                        distances.insert(next_p, c + 1);
                        to_check.push(HeapPosition {
                            pos: next_p,
                            cost: c + 1,
                        });
                    }
                }
            }

            if distances.get(&(n, n)).is_some() {
                None
            } else {
                Some(to_remove)
            }
        })
        .unwrap();
    println!("{:?}", a);
    0
}
