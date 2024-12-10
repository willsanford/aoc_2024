use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> u64 {
    let board: HashMap<(i32, i32), u32> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c.to_digit(10).unwrap()))
        })
        .flatten()
        .collect::<HashMap<(i32, i32), u32>>();

    let starts: Vec<(i32, i32)> = board
        .iter()
        .filter(|(_, val)| **val == 0)
        .map(|(pos, _)| *pos)
        .collect::<Vec<(i32, i32)>>();

    let dirs = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut tot: u64 = 0;
    for start in starts {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut poss: Vec<(u32, (i32, i32))> = vec![(0, start)];

        while let Some((val, pos)) = poss.pop() {
            visited.insert(pos);

            if val == 9 {
                tot += 1;
            }
            for dir in &dirs {
                let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if let Some(next_val) = board.get(&next_pos) {
                    if *next_val == val + 1 && !visited.contains(&next_pos) {
                        poss.push((*next_val, (next_pos)));
                    }
                }
            }
        }
    }
    tot
}

pub fn part2(input: String) -> u64 {
    let board: HashMap<(i32, i32), u32> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c.to_digit(10).unwrap()))
        })
        .flatten()
        .collect::<HashMap<(i32, i32), u32>>();

    let starts: Vec<(i32, i32)> = board
        .iter()
        .filter(|(_, val)| **val == 0)
        .map(|(pos, _)| *pos)
        .collect::<Vec<(i32, i32)>>();

    let dirs = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut tot: u64 = 0;
    for start in starts {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut poss: Vec<(u32, (i32, i32))> = vec![(0, start)];

        while let Some((val, pos)) = poss.pop() {
            visited.insert(pos);

            if val == 9 {
                tot += 1;
            }
            for dir in &dirs {
                let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if let Some(next_val) = board.get(&next_pos) {
                    if *next_val == val + 1 {
                        poss.push((*next_val, (next_pos)));
                    }
                }
            }
        }
    }
    tot
}
