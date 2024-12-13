fn f(a: (u64, u64), b: (u64, u64), total: (u64, u64)) -> Option<u64> {
    let pb: u64 = ((total.1 as f64 - (total.0 as f64 * a.1 as f64 / a.0 as f64))
        / (b.1 as f64 - (b.0 as f64 * a.1 as f64 / a.0 as f64)))
        .round() as u64;
    let pa: u64 = ((total.0 as f64 - (pb as f64 * b.0 as f64)) / a.0 as f64).round() as u64;

    println!("{},{}", pa, pb);
    if total == (pb * b.0 + pa * a.0, pb * b.1 + pa * a.1) {
        Some(3_u64 * pa + pb)
    } else {
        None
    }
}

fn parse_game(input: &str) -> ((u64, u64), (u64, u64), (u64, u64)) {
    let data = input
        .lines()
        .map(|line| {
            let (_, data) = line.split_once(": ").unwrap();
            let (a, b) = data.split_once(", ").unwrap();
            let (_, a_data) = a.split_at(2);
            let (_, b_data) = b.split_at(2);
            (
                a_data.parse::<u64>().unwrap(),
                b_data.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();
    (data[0], data[1], data[2])
}
pub fn part1(input: String) -> u64 {
    let games: Vec<((u64, u64), (u64, u64), (u64, u64))> = input
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(|group| parse_game(group))
        .collect();
    games
        .iter()
        .map(|game| f(game.0, game.1, game.2))
        .filter(|output| output.is_some())
        .map(|output| output.unwrap())
        .sum()
}

pub fn part2(input: String) -> u64 {
    let games: Vec<((u64, u64), (u64, u64), (u64, u64))> = input
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(|group| parse_game(group))
        .collect();
    games
        .iter()
        .map(|game| {
            f(
                game.0,
                game.1,
                (game.2 .0 + 10000000000000, game.2 .1 + 10000000000000),
            )
        })
        .filter(|output| output.is_some())
        .map(|output| output.unwrap())
        .sum()
}
