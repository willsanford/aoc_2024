use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
pub fn part1(input: String) -> u64 {
    let (unparsed_ordering, unparsed_updates) = input.split_once("\n\n").unwrap();
    let ordering: Vec<(u64, u64)> = unparsed_ordering
        .to_string()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, second) = line.split_once('|').unwrap();
            (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<HashMap<u64, usize>> = unparsed_updates
        .to_string()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|v| v.parse::<u64>().unwrap())
                .enumerate()
                .map(|(i, val)| (val, i))
                .collect::<HashMap<u64, usize>>()
        })
        .collect();

    updates
        .iter()
        .filter(|update| {
            ordering
                .iter()
                .find(
                    |(first, second)| match (update.get(first), update.get(second)) {
                        (Some(first_i), Some(second_i)) => second_i < first_i,
                        _ => false,
                    },
                )
                .is_none()
        })
        .map(|line| {
            let ind = line.len() / 2;
            line.iter().find(|(_, i)| ind == **i).unwrap().0
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let (unparsed_ordering, unparsed_updates) = input.split_once("\n\n").unwrap();
    let ordering: Vec<(u64, u64)> = unparsed_ordering
        .to_string()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, second) = line.split_once('|').unwrap();
            (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<HashMap<u64, usize>> = unparsed_updates
        .to_string()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|v| v.parse::<u64>().unwrap())
                .enumerate()
                .map(|(i, val)| (val, i))
                .collect::<HashMap<u64, usize>>()
        })
        .collect();

    let incorrect_updates: Vec<HashMap<u64, usize>> = updates
        .iter()
        .filter(|update| {
            ordering
                .iter()
                .find(
                    |(first, second)| match (update.get(first), update.get(second)) {
                        (Some(first_i), Some(second_i)) => second_i < first_i,
                        _ => false,
                    },
                )
                .is_some()
        })
        .map(|u| u.clone())
        .collect();

    let mut order_left: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut order_right: HashMap<u64, HashSet<u64>> = HashMap::new();
    for (left, right) in ordering {
        order_right
            .entry(left)
            .or_insert(HashSet::from([right]))
            .insert(right);
        order_left
            .entry(right)
            .or_insert(HashSet::from([left]))
            .insert(left);
    }

    let mut sum = 0;

    for incorrect_update in &incorrect_updates {
        let mut line: Vec<u64> = vec![1; incorrect_update.len()];
        for (val, i) in incorrect_update {
            line[*i] = *val;
        }
        line.sort_by(|a, b| {
            if let Some(vals) = order_left.get(&a) {
                if vals.contains(&b) {
                    return Ordering::Less;
                }
            }
            Ordering::Greater
        });
        sum += line[line.len() / 2]
    }
    sum
}
