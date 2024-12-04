pub fn part1(input: String) -> u64 {
    let vals = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    vals.iter()
        .filter(|arr| -> bool {
            let diffs = arr
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect::<Vec<i32>>();
            let not_steep = diffs
                .iter()
                .all(|val| (*val != 0) & (*val < 4) & (*val > -4));
            let same_dir = diffs
                .windows(2)
                .fold(true, |same, vals| same & (vals[0] * vals[1] > 0));
            not_steep & same_dir
        })
        .count() as u64
}

pub fn part2(input: String) -> u64 {
    let vals = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // For each value in the array, drop it and try again
    vals.iter()
        .filter(|_arr| -> bool {
            (0.._arr.len()).any(|drop_ind| {
                let arr = _arr
                    .iter()
                    .enumerate()
                    .filter(move |(i, _)| *i != drop_ind)
                    .map(|(_, val)| val)
                    .collect::<Vec<&i32>>();
                let diffs = arr
                    .windows(2)
                    .map(|vals| vals[1] - vals[0])
                    .collect::<Vec<i32>>();
                let not_steep = diffs
                    .iter()
                    .all(|val| (*val != 0) & (*val < 4) & (*val > -4));
                let same_dir = diffs
                    .windows(2)
                    .fold(true, |same, vals| same & (vals[0] * vals[1] > 0));
                not_steep & same_dir
            })
        })
        .count() as u64
}
