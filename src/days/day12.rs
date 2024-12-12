use std::collections::{HashMap, HashSet};

fn touches(a: &(i32, i32), b: &(i32, i32)) -> bool {
    match ((a.0 - b.0).abs(), (a.1 - b.1).abs()) {
        (0, 1) | (1, 0) => true,
        _ => false,
    }
}

fn get_sections(vals: &Vec<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
    let mut sections: Vec<HashSet<(i32, i32)>> = Vec::new();

    for v in vals {
        let mut adjacent_groups: Vec<usize> = Vec::new();

        for (i, group) in sections.iter().enumerate() {
            for group_member in group {
                if touches(&v, group_member) {
                    adjacent_groups.push(i);
                    continue;
                }
            }
        }

        match adjacent_groups.len() {
            0 => sections.push(HashSet::from_iter(vec![*v])),
            1 => {
                let _ = sections[adjacent_groups[0]].insert(*v);
            }
            _ => {
                if let Some((first, others)) = adjacent_groups.split_first() {
                    let mut first_section = sections[*first].clone();
                    for other_group in others {
                        first_section.extend(sections[*other_group].clone());
                        sections[*other_group] = HashSet::new();
                    }
                    first_section.insert(*v);
                    sections[*first] = first_section;
                }
            }
        }
    }
    sections
        .iter()
        .map(|v| Vec::from_iter(v.clone()))
        .collect::<Vec<Vec<(i32, i32)>>>()
}

fn calc_cost(vals: &Vec<(i32, i32)>) -> u64 {
    let mut fences: u64 = 0;
    for v in vals {
        let mut open_edge = 4;
        for other_v in vals {
            if (*v != *other_v) && touches(v, other_v) {
                open_edge -= 1;
            }
        }
        fences += open_edge;
    }

    fences * vals.len() as u64
}
pub fn part1(input: String) -> u64 {
    let mut board: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            board
                .entry(c)
                .or_insert(Vec::new())
                .push((i as i32, j as i32));
        }
    }

    let all_groups = board
        .iter()
        .flat_map(|(_, vec)| get_sections(vec))
        .collect::<Vec<Vec<(i32, i32)>>>();

    all_groups.iter().map(|v| calc_cost(v)).sum()
}

fn calc_cost_p2(vals: &Vec<(i32, i32)>) -> u64 {
    let spaces: HashSet<(i32, i32)> = HashSet::from_iter(vals.iter().map(|v| *v));

    // Get all the side on the right
    let unique_rows: HashSet<i32> = HashSet::from_iter(vals.iter().map(|(row, _)| *row));
    let unique_cols: HashSet<i32> = HashSet::from_iter(vals.iter().map(|(_, col)| *col));

    let mut col_values_r: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut col_values_l: HashMap<i32, Vec<i32>> = HashMap::new();
    for row in unique_rows {
        for (r, c) in vals.iter().filter(|(r_, _)| *r_ == row) {
            if !spaces.contains(&(*r, c - 1)) {
                col_values_l.entry(*c).or_insert(Vec::new()).push(*r);
            }
            if !spaces.contains(&(*r, c + 1)) {
                col_values_r.entry(*c).or_insert(Vec::new()).push(*r);
            }
        }
    }

    let mut row_values_d: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut row_values_u: HashMap<i32, Vec<i32>> = HashMap::new();
    for col in unique_cols {
        for (r, c) in vals.iter().filter(|(_, _c)| *_c == col) {
            if !spaces.contains(&(r - 1, *c)) {
                row_values_u.entry(*r).or_insert(Vec::new()).push(*c);
            }
            if !spaces.contains(&(r + 1, *c)) {
                row_values_d.entry(*r).or_insert(Vec::new()).push(*c);
            }
        }
    }

    let mut num_sides: u64 = 0;
    for cvs in vec![col_values_r, col_values_l, row_values_d, row_values_u] {
        for (_, mut vec) in cvs {
            vec.sort();
            num_sides += vec
                .windows(2)
                .map(|adjacent| adjacent[1] - adjacent[0])
                .filter(|diffs| *diffs > 1)
                .count() as u64
                + 1;
        }
    }
    num_sides * vals.len() as u64
}

pub fn part2(input: String) -> u64 {
    let mut board: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            board
                .entry(c)
                .or_insert(Vec::new())
                .push((i as i32, j as i32));
        }
    }

    let all_groups = board
        .iter()
        .flat_map(|(_, vec)| get_sections(vec))
        .collect::<Vec<Vec<(i32, i32)>>>();

    all_groups.iter().map(|v| calc_cost_p2(v)).sum()
}
