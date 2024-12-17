use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HeapPosition {
    dir: usize,
    cost: u64,
    pos: (i32, i32),
}

impl Ord for HeapPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for HeapPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: String) -> u64 {
    let _board: HashMap<(i32, i32), char> =
        HashMap::from_iter(input.lines().enumerate().flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                let pos = (i as i32, j as i32);
                match c {
                    '#' => None,
                    _ => Some((pos, c)),
                }
            })
        }));

    let start = _board
        .iter()
        .find_map(|(p, c)| match c {
            'S' => Some(*p),
            _ => None,
        })
        .unwrap();

    let end = _board
        .iter()
        .find_map(|(p, c)| match c {
            'E' => Some(*p),
            _ => None,
        })
        .unwrap();

    let board: HashSet<(i32, i32)> = HashSet::from_iter(_board.iter().map(|(p, c)| *p));

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut distances: HashMap<((i32, i32), (i32, i32)), u64> = HashMap::new();

    let mut heap: BinaryHeap<HeapPosition> = BinaryHeap::new();

    heap.push(HeapPosition {
        pos: start,
        dir: 0,
        cost: 0,
    });

    while let Some(p) = heap.pop() {
        // Check all directions
        for (dd, c) in vec![(0, 1_u64), (1, 1001_u64), (-1, 1001_u64)] {
            let dir_i = (p.dir as i32 + dd).rem_euclid(dirs.len() as i32) as usize;
            let new_dir = dirs[dir_i];
            let next_p = (p.pos.0 + new_dir.0, p.pos.1 + new_dir.1);
            let k = &(next_p, new_dir);
            if board.contains(&next_p)
                && distances
                    .get(k)
                    .map_or_else(|| true, |cost| p.cost + c < *cost)
            {
                distances.insert(*k, p.cost + c);
                heap.push(HeapPosition {
                    dir: dir_i,
                    cost: p.cost + c,
                    pos: next_p,
                });
            }
        }
    }

    distances
        .iter()
        .filter_map(|((pos, _), cost)| match *pos == end {
            true => Some(*cost),
            _ => None,
        })
        .min()
        .unwrap()
}

fn print_board(board: &Vec<(i32, i32)>) {
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


pub fn part2(input: String) -> u64 {let _board: HashMap<(i32, i32), char> =
    HashMap::from_iter(input.lines().enumerate().flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                let pos = (i as i32, j as i32);
                match c {
                    '#' => None,
                    _ => Some((pos, c)),
                }
            })
        }));

    let start = _board
        .iter()
        .find_map(|(p, c)| match c {
            'S' => Some(*p),
            _ => None,
        })
        .unwrap();

    let end = _board
        .iter()
        .find_map(|(p, c)| match c {
            'E' => Some(*p),
            _ => None,
        })
        .unwrap();

    let board: HashSet<(i32, i32)> = HashSet::from_iter(_board.iter().map(|(p, _)| *p));

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut distances: HashMap<((i32, i32), (i32, i32)), u64> = HashMap::new();
    let mut heap: BinaryHeap<HeapPosition> = BinaryHeap::new();
    let mut backtrace: HashMap<((i32, i32), (i32, i32)), HashSet<((i32, i32), (i32, i32))>> = HashMap::new();

    heap.push(HeapPosition {
        pos: start,
        dir: 0,
        cost: 0,
    });

    while let Some(p) = heap.pop() {
        // Check all directions
        for (dd, c) in vec![(0, 1_u64), (1, 1001_u64), (-1, 1001_u64)] {
            let dir_i = (p.dir as i32 + dd).rem_euclid(dirs.len() as i32) as usize;
            let new_dir = dirs[dir_i];
            let next_p = (p.pos.0 + new_dir.0, p.pos.1 + new_dir.1);
            let k = &(next_p, new_dir);
            if board.contains(&next_p){
                match distances.get(k) {
                    // We have not seen it yet
                    None => {
                        heap.push(HeapPosition {
                            dir: dir_i,
                            cost: p.cost + c,
                            pos: next_p,
                        });
                        backtrace.entry(*k).or_insert(HashSet::new()).insert((p.pos, dirs[p.dir]));
                        distances.insert(*k, p.cost + c);
                    },
                    Some(cost) => {
                        match cost.cmp(&(p.cost + c)) {
                            Ordering::Less => {}, 
                            Ordering::Equal => {
                                heap.push(HeapPosition {
                                    dir: dir_i,
                                    cost: p.cost + c,
                                    pos: next_p,
                                });
                                backtrace.entry(*k).or_insert(HashSet::new()).insert((p.pos, dirs[p.dir]));
                                distances.insert(*k, p.cost + c);
                            }
                            Ordering::Greater => {
                                heap.push(HeapPosition {
                                    dir: dir_i,
                                    cost: p.cost + c,
                                    pos: next_p,
                                });
                                backtrace.insert(*k, HashSet::from_iter(vec![(p.pos, dirs[p.dir])]));
                                distances.insert(*k, p.cost + c);
                            } 
                        }
                    }
                }
            }
        }
    }
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let min_val = distances
        .iter()
        .filter_map(|((pos, _), cost)| {
            match *pos == end {
                true => Some(cost),
                _ => None
            }
        }).min().unwrap();

    let mut to_check: Vec<((i32, i32), (i32, i32))> = distances
        .iter()
        .filter_map(|(p, cost)| {
            match (p.0 == end, cost == min_val)  {
                (true, true) => Some(*p),
                _ => None
            }
        }).collect();

    while let Some(p) = to_check.pop(){
        seen.insert(p.0);
        if let Some(prevs) = backtrace.get(&p) {
            for prev in prevs {        
                to_check.push(*prev);
            }

        }
    }
    print_board(&Vec::from_iter(seen.iter().map(|a|*a)));
    seen.len() as u64
}
