pub fn part1(input: String) -> u64 {
    let data = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| -> Vec<char> { line.chars().collect::<Vec<char>>() })
        .collect::<Vec<Vec<char>>>();

    let mut dirs: Vec<(i32, i32)> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            dirs.push((i, j));
        }
    }

    let letters: Vec<(char, i32)> = vec![('M', 1), ('A', 2), ('S', 3)];
    let mut tot: u64 = 0;

    for i in 0..data.len() as i32 {
        for j in 0..data[0].len() as i32 {
            if data[i as usize][j as usize] != 'X' {
                continue;
            }

            let d = dirs.clone();
            for (di, dj) in &d {
                let mut valid = true;
                for (c, dis) in &letters {
                    if !valid {
                        continue;
                    }
                    // Check that this is a valid index
                    let ii = i + dis * di;
                    let jj = j + dis * dj;
                    if ii < 0 || ii >= data.len() as i32 || jj < 0 || jj >= data[0].len() as i32 {
                        valid = false;
                        continue;
                    }

                    if data[ii as usize][jj as usize] != *c {
                        valid = false;
                        continue;
                    }
                }
                if valid {
                    tot += 1;
                }
            }
        }
    }
    tot
}

pub fn part2(input: String) -> u64 {
    let data = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| -> Vec<char> { line.chars().collect::<Vec<char>>() })
        .collect::<Vec<Vec<char>>>();
    let mut tot = 0;

    for i in 1..(data.len() as i32) - 1 {
        for j in 1..(data.len() as i32) - 1 {
            if data[i as usize][j as usize] != 'A' {
                continue;
            }
            let tr = data[(i + 1) as usize][(j + 1) as usize];
            let bl = data[(i - 1) as usize][(j - 1) as usize];
            let tl = data[(i + 1) as usize][(j - 1) as usize];
            let br = data[(i - 1) as usize][(j + 1) as usize];
            let first = match (tr, bl) {
                ('M', 'S') | ('S', 'M') => true,
                _ => false,
            };

            let second = match (tl, br) {
                ('M', 'S') | ('S', 'M') => true,
                _ => false,
            };

            if first && second {
                tot += 1;
            }
        }
    }
    tot
}
