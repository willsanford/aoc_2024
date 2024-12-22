use std::collections::HashMap;
fn next_secret(seed: u64) -> u64 {
    let mut s = seed;
    s = (s ^ (s * 64) % 16777216);
    s = (s ^ (s.div_euclid(32)) % 16777216);
    s = (s ^ (s * 2048) % 16777216);
    s
}
pub fn part1(input: String) -> u64 {
    input
        .lines()
        .filter_map(|c| c.parse::<u64>().ok())
        .map(|_v| {
            let mut v = _v;
            for _ in (0..2000) {
                v = next_secret(v);
            }
            v
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    *input
        .lines()
        .filter_map(|c| c.parse::<u64>().ok())
        .flat_map(|start| {
            let mut changes: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();
            let mut current: (i64, i64, i64, i64) = (0, 0, 0, 0);
            let mut v = start;
            for i in 0..2000 {
                let next_v = next_secret(v);
                let diff = (next_v % 10) as i64 - (v % 10) as i64;
                current = (current.1, current.2, current.3, diff);
                if i >= 3 && !changes.contains_key(&current) {
                    changes.insert(current, next_v % 10);
                }
                v = next_v;
            }
            changes
        })
        .fold(
            HashMap::new(),
            |mut combined: HashMap<(i64, i64, i64, i64), u64>, (k, v)| {
                combined.entry(k).and_modify(|e| *e += v).or_insert(v);
                combined
            },
        )
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .1
}
