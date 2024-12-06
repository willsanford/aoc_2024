use std::collections::{HashMap, HashSet};
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

fn update_orderings(
    knowns: HashMap<u64, (Vec<u64>, Vec<u64>)>,
    updates: HashMap<u64, usize>,
) -> HashMap<u64, usize> {
    HashMap::new()
}

fn get_knowns(
    ordering: Vec<(u64, u64)>,
    updates: &Vec<HashMap<u64, usize>>,
) -> HashMap<u64, (Vec<u64>, Vec<u64>)> {
    let (left_graph, right_graph) = build_graphs(&ordering);

    let mut keys: HashSet<u64> = HashSet::new();
    for map in updates {
        for key in map.keys() {
            keys.insert(*key);
        }
    }
    // Total orderings that we know of. val: (left, right, unknown)
    let knowns: HashMap<u64, (Vec<u64>, Vec<u64>)> = keys
        .iter()
        .map(|k| {
            (
                *k,
                (
                    get_reachable(*k, left_graph.clone()),
                    get_reachable(*k, right_graph.clone()),
                ),
            )
        })
        .collect();

    knowns
}

fn build_graphs(ordering: &Vec<(u64, u64)>) -> (HashMap<u64, Vec<u64>>, HashMap<u64, Vec<u64>>) {
    let mut left_graph: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut right_graph: HashMap<u64, HashSet<u64>> = HashMap::new();

    for (left, right) in ordering {
        right_graph
            .entry(*left)
            .or_insert(HashSet::from([*right]))
            .insert(*right);
        left_graph
            .entry(*right)
            .or_insert(HashSet::from([*left]))
            .insert(*left);
    }

    let l = left_graph
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|v| *v).collect::<Vec<u64>>()))
        .collect();
    let r = right_graph
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|v| *v).collect::<Vec<u64>>()))
        .collect();
    (l, r)
}

fn get_reachable(seed: u64, graph: HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut seen: HashSet<u64> = HashSet::new();
    let mut to_check: Vec<u64> = vec![seed];

    while let Some(next) = to_check.pop() {
        match graph.get(&next) {
            Some(v) => {
                for v_ in v {
                    if !seen.contains(v_) {
                        seen.insert(*v_);
                        to_check.push(*v_);
                    }
                }
                // seen.extend(v.clone());
                // to_check.extend(v.clone());
            }
            None => {}
        }
    }

    // Return the uni
    seen.iter().map(|k| *k).collect::<Vec<u64>>()
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

    let knowns = get_knowns(ordering, &updates);
    println!("{:?}", knowns);
    let mut tot = 0;

    for inc_update in &incorrect_updates {
        for (k, _) in inc_update {
            if let Some((l, r)) = knowns.get(&k) {
                let l_size = l.iter().filter(|v| inc_update.contains_key(v)).count();
                let r_size = r.iter().filter(|v| inc_update.contains_key(v)).count();
                if l_size == r_size {
                    tot += k;
                }
            }
        }
    }
    tot
    // println!("{:?}", knowns);

    // incorrect_updates
    //     .iter()
    //     .map(|u| update_orderings(knowns.clone(), u.clone()))
    //     .map(|line| {
    //         let ind = line.len() / 2;
    //         line.iter().find(|(_, i)| ind == **i).unwrap().0.clone()
    //     })
    //     .sum()
}
