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

fn update_pos_p2(pos: (usize, usize), dir: &Dir, n: usize, m: usize) -> Option<(usize, usize)> {
    use Dir::*;
    match dir {
        U => {
            if (pos.0 as i32 - 1) < 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        D => {
            if pos.0 + 1 >= n {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
        L => {
            if (pos.1 as i32 - 1) < 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        R => {
            if pos.1 + 1 >= m {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
    }
}

fn check_cycle(start: (usize, usize), board: &Vec<Vec<char>>) -> bool {
    let n = board.len();
    let m = board[0].len();
    let mut dir: Dir = Dir::U;
    let mut pos: (usize, usize) = start;
    let mut visited_pos: HashSet<((usize, usize), Dir)> = HashSet::new();
    let mut cycle = false;

    loop {
        if visited_pos.contains(&(pos, dir.clone())) {
            cycle = true;
            break;
        }
        visited_pos.insert((pos, dir.clone()));

        let next_pos = update_pos_p2(pos, &dir, n, m);
        if next_pos.is_none() {
            break;
        }

        let np = next_pos.unwrap();

        if board[np.0][np.1] == '#' {
            dir = update_dir(dir);
        } else {
            pos = np;
        }
    }
    cycle
}

pub fn part2(input: String) -> u64 {
    let mut board = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let n = board.len();
    let m = board[0].len();

    let mut start: (usize, usize) = (0, 0);
    for _n in 0..n {
        for _m in 0..m {
            if board[_n][_m] == '^' {
                start = (_n, _m);
                board[_n][_m] = '.';
            }
        }
    }

    let mut count: u64 = 0;
    for _n in 0..n {
        for _m in 0..m {
            if board[_n][_m] == '#' || (_n, _m) == start {
                continue;
            }
            board[_n][_m] = '#';
            if check_cycle(start, &board) {
                count += 1;
            }
            board[_n][_m] = '.';
        }
    }

    count
}
