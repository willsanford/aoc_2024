use std::collections::HashMap;

fn p1_helper(s: String, towels: &Vec<String>, cache: &mut HashMap<String, bool>) -> bool {
    if let Some(b_) = cache.get(&s) {
        return *b_;
    }
    let b = if s.is_empty() {
        true
    } else {
        towels.iter().fold(false, |b, t| {
            b | (s.starts_with(t) && p1_helper(s.chars().skip(t.len()).collect(), towels, cache))
        })
    };
    cache.insert(s, b);
    b
}
pub fn part1(input: String) -> u64 {
    let (towels_str, pattern) = input.split_once("\n\n").unwrap();
    let towels: Vec<String> = towels_str.split(", ").map(|s| s.to_string()).collect();

    let patterns: Vec<String> = pattern.lines().map(|line| line.to_string()).collect();

    let mut cache = HashMap::new();
    patterns
        .iter()
        .filter(|s| p1_helper(s.to_string(), &towels, &mut cache))
        .count() as u64
}

fn p2_helper(s: String, towels: &Vec<String>, cache: &mut HashMap<String, u64>) -> u64 {
    if let Some(b_) = cache.get(&s) {
        return *b_;
    }
    let t = if s.is_empty() {
        1
    } else {
        towels.iter().fold(0_u64, |b, t| {
            if s.starts_with(t) {
                b + p2_helper(s.chars().skip(t.len()).collect(), towels, cache)
            } else {
                b
            }
        })
    };
    cache.insert(s, t);
    t
}
pub fn part2(input: String) -> u64 {
    let (towels_str, pattern) = input.split_once("\n\n").unwrap();
    let towels: Vec<String> = towels_str.split(", ").map(|s| s.to_string()).collect();

    let patterns: Vec<String> = pattern.lines().map(|line| line.to_string()).collect();

    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|s| p2_helper(s.to_string(), &towels, &mut cache))
        .sum()
}
