use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

pub fn part1(input: String) -> u64 {
    let mut board: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(move |(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect();

    let start: (i32, i32) = board
        .iter()
        .find_map(|(p, c)| match *c {
            'S' => Some(*p),
            _ => None,
        })
        .unwrap();
    let end: (i32, i32) = board
        .iter()
        .find_map(|(p, c)| match *c {
            'E' => Some(*p),
            _ => None,
        })
        .unwrap();

    board = board
        .iter()
        .map(|(p, c)| match *c {
            'S' | 'E' => (*p, '.'),
            _ => (*p, *c),
        })
        .collect();

    let mut distances: HashMap<(i32, i32), u64> = HashMap::new();
    distances.insert(start, 0);
    let mut current: (i32, i32) = start;
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    loop {
        for (di, dj) in &dirs {
            let next = (current.0 + di, current.1 + dj);

            match (board.get(&next), distances.get(&next)) {
                (Some(c), None) if *c == '.' => {
                    distances.insert(next, distances.get(&current).unwrap() + 1);
                    current = next;
                    break;
                }
                _ => {}
            }
        }
        if current == end {
            break;
        };
    }

    board
        .iter()
        .filter(|(_, c)| **c == '#')
        .flat_map(|(p, _)| {
            (0..=1).map(|i| {
                let (s1, s2) = (
                    (p.0 + dirs[2 * i].0, p.1 + dirs[2 * i].1),
                    (p.0 + dirs[2 * i + 1].0, p.1 + dirs[2 * i + 1].1),
                );

                match (
                    board.get(&s1),
                    board.get(&s2),
                    distances.get(&s1),
                    distances.get(&s2),
                ) {
                    (Some('.'), Some('.'), Some(d1), Some(d2)) => d1.abs_diff(*d2) as i64 - 2,
                    _ => 0,
                }
            })
        })
        .filter(|c| *c >= 100)
        .count() as u64
}

pub fn part2(input: String) -> u64 {
    let mut board: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(move |(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect();

    let start: (i32, i32) = board
        .iter()
        .find_map(|(p, c)| match *c {
            'S' => Some(*p),
            _ => None,
        })
        .unwrap();
    let end: (i32, i32) = board
        .iter()
        .find_map(|(p, c)| match *c {
            'E' => Some(*p),
            _ => None,
        })
        .unwrap();

    board = board
        .iter()
        .map(|(p, c)| match *c {
            'S' | 'E' => (*p, '.'),
            _ => (*p, *c),
        })
        .collect();

    let mut distances: HashMap<(i32, i32), u64> = HashMap::new();
    distances.insert(start, 0);
    let mut current: (i32, i32) = start;
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    loop {
        for (di, dj) in &dirs {
            let next = (current.0 + di, current.1 + dj);

            match (board.get(&next), distances.get(&next)) {
                (Some(c), None) if *c == '.' => {
                    distances.insert(next, distances.get(&current).unwrap() + 1);
                    current = next;
                    break;
                }
                _ => {}
            }
        }
        if current == end {
            break;
        };
    }

    // Get the path
    iproduct!(&distances, &distances)
        .filter(|((pos1, d1), (pos2, d2))| {
            let a = (**d1 as i64 - **d2 as i64 - 2 >= 50)
                && (pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1) <= 20);
            if a {
                println!("{:?} -> {:?}", pos1, pos2);
            }
            a
        })
        .count() as u64
}
