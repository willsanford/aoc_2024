use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    let mut arr: Vec<u64> = input
        .split(" ")
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..30 {
        arr = arr
            .iter()
            .map(|v| -> Vec<u64> {
                if *v == 0 {
                    return vec![1];
                }
                let s = v.to_string();
                let l = s.len();
                if l % 2 == 0 {
                    let (first, second) = s.split_at(l / 2);
                    return vec![
                        first.parse::<u64>().unwrap(),
                        second.parse::<u64>().unwrap(),
                    ];
                } else {
                    return vec![v * 2024];
                }
            })
            .flatten()
            .collect::<Vec<u64>>()
    }
    arr.len() as u64
}

fn increment(val: u64, cache: &mut HashMap<u64, (u64, Option<u64>)>) -> (u64, Option<u64>) {
    if let Some(out) = cache.get(&val) {
        return *out;
    }

    let s = val.to_string();

    let out: (u64, Option<u64>) = if val == 0 {
        (1, None)
    } else if s.len() % 2 == 0 {
        let (left, right) = s.split_at(s.len() / 2);
        (
            left.parse::<u64>().unwrap(),
            Some(right.parse::<u64>().unwrap()),
        )
    } else {
        (val * 2024, None)
    };

    cache.insert(val, out);

    out
}
pub fn part2(input: String) -> u64 {
    let arr: Vec<u64> = input
        .split(" ")
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut vals: HashMap<u64, u64> = HashMap::new();
    for v in arr {
        vals.entry(v).and_modify(|e| *e += 1_u64).or_insert(1);
    }

    let mut cache: HashMap<u64, (u64, Option<u64>)> = HashMap::new();

    for _ in 0..75 {
        let mut next: HashMap<u64, u64> = HashMap::new();
        for (k, v) in &vals {
            let (a, b_opt) = increment(*k, &mut cache);

            next.entry(a).and_modify(|e| *e += v).or_insert(*v);
            if let Some(b) = b_opt {
                next.entry(b).and_modify(|e| *e += v).or_insert(*v);
            }
        }

        vals = next;
    }

    vals.values().sum()
}
