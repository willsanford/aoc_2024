use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn update_pos(pos: (i32, i32), dir: &Dir) -> (i32, i32) {
    use Dir::*;
    match dir {
        U => (pos.0, pos.1 - 1),
        D => (pos.0, pos.1 + 1),
        L => (pos.0 - 1, pos.1),
        R => (pos.0 + 1, pos.1),
    }
}

fn update_dir(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        U => R,
        R => D,
        D => L,
        L => U,
    }
}
pub fn part1(input: String) -> u64 {
    use Dir::*;
    let mut board: HashMap<(i32, i32), char> = HashMap::new();
    let mut pos: (i32, i32) = (0, 0);
    let mut dir: Dir = U;
    for (j, line) in input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (i, c) in line.chars().enumerate() {
            let _pos = (i as i32, j as i32);
            if c == '^' {
                pos = _pos;
                board.insert(_pos, '.');
            } else {
                board.insert(_pos, c);
            }
        }
    }
    let n = *board.iter().map(|((_, n), _)| n).max().unwrap() as i32;
    let m = *board.iter().map(|((m, _), _)| m).max().unwrap() as i32;

    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    while board.contains_key(&pos) {
        visited_pos.insert(pos);
        let next_pos = update_pos(pos, &dir);
        println!("{:?}|{:?}", pos, next_pos);
        match board.get(&next_pos) {
            Some('.') => pos = next_pos,
            Some('#') => dir = update_dir(dir),
            _ => pos = next_pos,
        };
    }

    visited_pos.len() as u64
}

fn check_cycle(start: (i32, i32), board: HashMap<(i32, i32), char>) -> bool {
    let mut dir: Dir = Dir::U;
    let mut pos: (i32, i32) = start;
    let mut visited_pos: HashSet<((i32, i32), Dir)> = HashSet::new();
    let mut cycle = false;
    while board.contains_key(&pos) {
        if visited_pos.contains(&(pos, dir.clone())) {
            cycle = true;
            break;
        }
        visited_pos.insert((pos, dir.clone()));
        let next_pos = update_pos(pos, &dir);
        match board.get(&next_pos) {
            Some('.') => pos = next_pos,
            Some('#') => dir = update_dir(dir),
            _ => pos = next_pos,
        };
    }
    cycle
}

pub fn part2(input: String) -> u64 {
    use Dir::*;
    let mut board: HashMap<(i32, i32), char> = HashMap::new();
    let mut pos: (i32, i32) = (0, 0);
    for (j, line) in input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (i, c) in line.chars().enumerate() {
            let _pos = (i as i32, j as i32);
            if c == '^' {
                pos = _pos;
                board.insert(_pos, '.');
            } else {
                board.insert(_pos, c);
            }
        }
    }
    let n = *board.iter().map(|((_, n), _)| n).max().unwrap() as i32;
    let m = *board.iter().map(|((m, _), _)| m).max().unwrap() as i32;

    let mut count = 0;
    for n_ in 0..n + 1 {
        for m_ in 0..m + 1 {
            if (n_, m_) == pos {
                continue;
            }

            if let Some(c) = board.get(&(n_, m_)) {
                if *c == '.' {
                    let mut new_board = board.clone();
                    new_board.insert((n_, m_), '#');
                    if check_cycle(pos, new_board) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
