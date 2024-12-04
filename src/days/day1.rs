use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    let (mut first, mut second): (Vec<u64>, Vec<u64>) = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| -> (&str, &str) { line.split_once("   ").unwrap() })
        .map(|(first, second)| -> (u64, u64) {
            (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            )
        })
        .unzip();

    first.sort();
    second.sort();

    first
        .iter()
        .zip(second.iter())
        .map(|(n1, n2)| -> u64 { n1.abs_diff(n2.clone()) })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let (first, second): (Vec<u64>, Vec<u64>) = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| -> (&str, &str) { line.split_once("   ").unwrap() })
        .map(|(first, second)| -> (u64, u64) {
            (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            )
        })
        .unzip();
    let mut counts = HashMap::new();
    second.into_iter().for_each(|n| {
        counts.entry(n).and_modify(|_n| *_n += 1).or_insert(1);
    });
    first
        .iter()
        .map(|n| {
            n * match counts.get(n) {
                Some(_n) => _n,
                None => &0,
            }
        })
        .sum()
}
