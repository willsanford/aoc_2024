use itertools::iproduct;
use std::collections::{HashMap, HashSet};
pub fn part1(input: String) -> u64 {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(HashMap::new(), |mut map, (u, v)| {
            map.entry(v).or_insert(Vec::new()).push(u);
            map.entry(u).or_insert(Vec::new()).push(v);
            map
        });

    let groups: HashSet<Vec<&str>> = graph
        .iter()
        .flat_map(|(u, vs)| {
            // Check if the there are pairs of
            iproduct!(vs, vs)
                .filter(|(first, second)| {
                    graph.get(*first).unwrap().contains(&second)
                        && graph.get(*second).unwrap().contains(&first)
                })
                .map(|(first, second)| {
                    let mut vec: Vec<&str> = vec![first, second, u];
                    vec.sort();
                    vec
                })
        })
        .collect();

    groups
        .iter()
        .filter(|group| group.iter().any(|c| c.starts_with('t')))
        .count() as u64
}

pub fn part2(input: String) -> u64 {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(HashMap::new(), |mut map, (u, v)| {
            map.entry(v).or_insert(Vec::new()).push(u);
            map.entry(u).or_insert(Vec::new()).push(v);
            map
        });

    let groups: HashSet<Vec<&str>> = graph
        .iter()
        .flat_map(|(u, vs)| {
            // Check if the there are pairs of
            iproduct!(vs, vs)
                .filter(|(first, second)| {
                    graph.get(*first).unwrap().contains(&second)
                        && graph.get(*second).unwrap().contains(&first)
                })
                .map(|(first, second)| {
                    let mut vec: Vec<&str> = vec![first, second, u];
                    vec.sort();
                    vec
                })
        })
        .collect();

    groups
        .iter()
        .filter(|group| group.iter().any(|c| c.starts_with('t')))
        .count() as u64
}
