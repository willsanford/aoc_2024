use std::collections::{HashMap, HashSet};

fn print_board(board: &HashMap<(i32, i32), char>) {
   let n = board.keys().map(|(i, _)| *i).max().unwrap() as usize + 1;
   let m = board.keys().map(|(_, j)| *j).max().unwrap() as usize + 1;

   let mut board_str: Vec<Vec<char>> = vec![vec![' '; m]; n];
   for ((i, j), c) in board {
      board_str[*i as usize][*j as usize] = *c;
   }
   for line in board_str {
     println!("{}", line.iter().collect::<String>()); 
   }
}

pub fn part1(input: String) -> u64 {
   let (board_str, moves_str) = input.split_once("\n\n").unwrap();


   let mut board: HashMap<(i32, i32), char> = HashMap::from_iter(
      board_str.lines().enumerate().flat_map(|(i, line)| {
         line.chars().enumerate().map(move |(j, c)| {
            ((i as i32, j as i32), c)
         })
      })
   );

   let moves: Vec<(i32, i32)> = moves_str.lines().flat_map(|line| {
      line.chars().map(|c|{
         match c {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => (0,0)
         }
      })
   }).collect();

   let mut current_pos = board.iter().find_map(|((i, j), c)| {
      match c {
         '@' => Some((*i, *j)),
         _ => None
      }
   }).unwrap();


   for (di, dj) in moves {
      let mut next_pos = (current_pos.0 + di, current_pos.1 + dj);
      current_pos = match board.get(&next_pos) {
         Some('.') | Some('@') => next_pos,
         Some('#') => current_pos,
         Some('O') => {
            let mut final_pos = (next_pos.0 + di, next_pos.1 + dj);
            while let Some(c) = board.get(&final_pos) {
               if *c == '.' || *c == '@' {
                  board.insert(next_pos, '.');
                  board.insert(final_pos, 'O');
                  break;
               } else if *c == '#' {
                  next_pos = current_pos;
                  break;
               }
               
               final_pos = (final_pos.0 + di, final_pos.1 + dj);
            }
            next_pos
         },
         _ => {
            println!("skjldhjksdhfksjhdf");
            current_pos
         }
      };
      // print_board(&board);
   } 
   board.iter().filter_map(|((i, j), c)| {
      match c {
         'O' => Some(100 * i + j),
         _ => None
      }
   }).sum::<i32>() as u64
}

pub fn part2(input: String) -> u64 {
   let (board_str, moves_str) = input.split_once("\n\n").unwrap();

   let mut board: HashMap<(i32, i32), char> = HashMap::from_iter(
      board_str.lines().enumerate().flat_map(|(i, line)| {
         line.chars().enumerate().flat_map(move |(j, c)| {
            let cs = match c {
               '#' => ('#', '#'),
               '@' => ('@', '.'),
               'O' => ('[', ']'),
               _ => ('.','.')
            };
            vec![
               ((i as i32, 2 * (j as i32)), cs.0),
               ((i as i32, 2 * (j as i32) + 1), cs.1)
            ]
         })
      })
   );

   let moves: Vec<(i32, i32)> = moves_str.lines().flat_map(|line| {
      line.chars().filter_map(|c|{
         match c {
            '>' => Some((0, 1)),
            '<' => Some((0, -1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None 
         }
      })
   }).collect();

   let mut current_pos = board.iter().find_map(|((i, j), c)| {
      match c {
         '@' => Some((*i, *j)),
         _ => None
      }
   }).unwrap();


   for (di, dj) in moves {
      println!("Move: {},{}", di, dj);
      let mut next_pos = (current_pos.0 + di, current_pos.1 + dj);
      current_pos = match board.get(&next_pos) {
         Some('.') | Some('@') => next_pos,
         Some('#') => current_pos,
         Some('[') | Some(']') => {
            // Moving laterally
            let mut effected_squares: HashSet<((i32, i32), char)> = HashSet::from_iter(vec![(next_pos, *board.get(&next_pos).unwrap())]);
            let mut to_move: bool = true;
            let mut final_pos = (next_pos.0 + di, next_pos.1 + dj);
            while let Some(c) = board.get(&final_pos) {
               if *c == '.' || *c == '@' {
                  break;
               } else if *c == '[' || *c == ']' {
                  // If we are moving up or down, then we need to check if this has the other half
                  // of the box effected
                  if dj == 0 {
                     let other_side: (i32, i32) = (final_pos.0, final_pos.1 + if *c == ']' {-1} else {1});
                     effected_squares.insert((other_side, if *c == ']' {'['} else {']'})); 
                  }
                  effected_squares.insert((final_pos, *c)); 
               } else {
                  to_move = false;
                  break;
               }
               final_pos = (final_pos.0 + di, final_pos.1 + dj);
            } 
            if to_move {
               for (p, c) in effected_squares {
                  board.insert(p, c);
               }
            } else {
               next_pos = current_pos;
            }
            next_pos
         }
         _ => {
            println!("skjldhjksdhfksjhdf");
            current_pos
         }
      };
      print_board(&board);
   } 
   board.iter().filter_map(|((i, j), c)| {
      match c {
         '[' => Some(100 * i + j),
         _ => None
      }
   }).sum::<i32>() as u64
}
