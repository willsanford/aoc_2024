pub fn part1(input: String) -> u64 {
    let vals: Vec<usize> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut files: Vec<i64> = vals
        .chunks(2)
        .enumerate()
        .map(|(id, chunk)| {
            let mut file = vec![id as i64; chunk[0]];
            if chunk.len() > 1 {
                file.extend_from_slice(vec![-1_i64; chunk[1]].as_slice());
            }
            file
        })
        .flatten()
        .collect::<Vec<i64>>();

    let free_spaces: Vec<usize> = files
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == -1_i64)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let used_spaces: Vec<usize> = files
        .iter()
        .enumerate()
        .filter(|(_, val)| **val >= 0)
        .map(|(i, _)| i)
        .rev()
        .collect::<Vec<usize>>();

    for (used, free) in used_spaces.iter().zip(free_spaces) {
        if *used < free {
            break;
        }

        files[free] = files[*used];
        files[*used] = -1_i64;
    }

    files
        .iter()
        .enumerate()
        .filter(|(_, val)| **val >= 0)
        .map(|(id, val)| id as u64 * *val as u64)
        .sum()
}

pub fn part2(input: String) -> u64 {
    let vals: Vec<usize> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut files: Vec<i64> = vals
        .chunks(2)
        .enumerate()
        .map(|(id, chunk)| {
            let mut file = vec![id as i64; chunk[0]];
            if chunk.len() > 1 {
                file.extend_from_slice(vec![-1_i64; chunk[1]].as_slice());
            }
            file
        })
        .flatten()
        .collect::<Vec<i64>>();

    let mut starts = vals.clone();
    starts.iter_mut().fold(0, |acc, x| {
        *x += acc;
        *x
    });
    starts.pop();
    starts.insert(0, 0);

    let regions: Vec<(usize, usize)> = starts
        .iter()
        .zip(vals.iter())
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<(usize, usize)>>();
    let mut used: Vec<(i64, usize, usize)> = Vec::new();
    let mut free: Vec<(usize, usize)> = Vec::new();
    for (i, val) in regions.iter().enumerate() {
        if i % 2 == 0 {
            used.push(((i / 2) as i64, val.0, val.1));
        } else {
            free.push(*val);
        }
    }

    for (val, start, len) in used.iter().rev() {
        for i in 0..free.len() {
            let mut free_start = free[i].0;
            let mut free_len = free[i].1;

            if free_start < *start && *len <= free_len {
                // Move the value in the file list
                for i in 0..*len {
                    files[free_start + i] = *val;
                    files[start + i] = -1_i64;
                }
                free_start += *len;
                free_len -= *len;
                free[i] = (free_start, free_len);
                break;
            }
        }
    }

    files
        .iter()
        .enumerate()
        .filter(|(_, val)| **val >= 0)
        .map(|(id, val)| id as u64 * *val as u64)
        .sum()
}
