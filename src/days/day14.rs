use scanf::sscanf;

pub fn part1(input: String) -> u64 {
    let (max_x, max_y) = (101, 103);
    let (mid_x, mid_y) = ((max_x / 2), (max_y / 2));
    let max_time = 100;
    input
        .lines()
        .map(|line| {
            let (pos, mov) = line.split_once(' ').unwrap();
            let (_, pos_) = pos.split_once("=").unwrap();
            let (_, mov_) = mov.split_once("=").unwrap();
            let (xstr, ystr) = pos_.split_once(",").unwrap();
            let (dxstr, dystr) = mov_.split_once(",").unwrap();

            (
                (xstr.parse::<i64>().unwrap(), ystr.parse::<i64>().unwrap()),
                (dxstr.parse::<i64>().unwrap(), dystr.parse::<i64>().unwrap()),
            )
        })
        .map(|((x, y), (dx, dy))| {
            (
                (x + max_time * dx).rem_euclid(max_x),
                (y + max_time * dy).rem_euclid(max_y),
            )
        })
        .fold(vec![0; 4], |mut qs, (x, y)| {
            if (x == mid_x || y == mid_y) {
                qs
            } else {
                let qx = (x < mid_x) as usize;
                let qy = (y < mid_y) as usize;
                qs[2 * qx + qy] += 1;
                qs
            }
        })
        .iter()
        .product::<u64>()
}

fn var(vals: Vec<i64>) -> f64 {
    let fvals = vals.iter().map(|v| *v as f64).collect::<Vec<f64>>();
    let n = vals.len() as f64;
    let mean: f64 = fvals.iter().sum();
    fvals.iter().map(|v| (*v - mean).powf(2_f64)).sum::<f64>() / n
}

pub fn part2(input: String) -> u64 {
    let (max_x, max_y) = (101, 103);
    let max_time = 10000;
    let robots: Vec<((i64, i64), (i64, i64))> = input
        .lines()
        .map(|line| {
            let (pos, mov) = line.split_once(' ').unwrap();
            let (_, pos_) = pos.split_once("=").unwrap();
            let (_, mov_) = mov.split_once("=").unwrap();
            let (xstr, ystr) = pos_.split_once(",").unwrap();
            let (dxstr, dystr) = mov_.split_once(",").unwrap();

            (
                (xstr.parse::<i64>().unwrap(), ystr.parse::<i64>().unwrap()),
                (dxstr.parse::<i64>().unwrap(), dystr.parse::<i64>().unwrap()),
            )
        })
        .collect();

    let time_var: Vec<(i64, f64)> = (1..=max_time)
        .map(|t| {
            let (xs, ys): (Vec<i64>, Vec<i64>) = robots
                .iter()
                .map(|((x, y), (dx, dy))| {
                    (
                        (x + t * dx).rem_euclid(max_x),
                        (y + t * dy).rem_euclid(max_y),
                    )
                })
                .unzip();
            (t, var(xs) + var(ys))
        })
        .collect();
    let min_var_time = time_var
        .iter()
        .min_by(|(_, a_var), (_, b_var)| a_var.total_cmp(b_var))
        .map(|(t, _)| *t)
        .unwrap() as u64;

    // Display the frame as this time

    let positions = robots.iter().map(|((x, y), (dx, dy))| {
        (
            (x + min_var_time as i64 * dx).rem_euclid(max_x),
            (y + min_var_time as i64 * dy).rem_euclid(max_y),
        )
    });

    let mut board: Vec<Vec<char>> = vec![vec![' '; max_x as usize]; max_y as usize];
    for (x, y) in positions {
        board[y as usize][x as usize] = '*';
    }
    let frame: String = board
        .iter()
        .flat_map(|line| {
            let mut l = line.clone();
            l.push('\n');
            l
        })
        .collect();

    println!("{}", frame);
    min_var_time
}
