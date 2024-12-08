use std::collections::{HashMap, HashSet};

fn get_antinodes(a: &(i32, i32), b: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
    let anti_a = (2 * a.0 - b.0, 2 * a.1 - b.1);
    let anti_b = (2 * b.0 - a.0, 2 * b.1 - a.1);
    (anti_a, anti_b)
}
pub fn part1(input: String) -> u64 {
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut m = 0;
    let mut n = 0;
    input.split('\n').filter(|line| !line.is_empty()).enumerate().for_each(|(i, line)| {
        n = n.max(i as i32);
        line.chars().enumerate().for_each(|(j, c)|{
            m = m.max(j as i32);
            if c != '.' {
                positions.entry(c).or_insert(vec![(i as i32, j as i32)]).push((i as i32, j as i32));
            }
        });
    });
    
        
    let mut unique_pos: HashSet<(i32, i32)> = HashSet::new();
    for (_, pos) in positions {
        for a in &pos {
            for b in &pos {
                if a == b {
                    continue;
                }
                let (first, second) = get_antinodes(a, b);
                if first.0 >= 0 && first.0 <= n  && first.1 >= 0 && first.1 <= m {
                    unique_pos.insert(first);
                } 
                if second.0 >= 0 && second.0 <= n && second.1 >= 0 && second.1 <= m {
                    unique_pos.insert(second);
                } 
            }
        }
    }

    unique_pos.len() as u64
}

fn get_antinodes_p2(a: &(i32, i32), b: &(i32, i32), n: i32, m: i32) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new(); 

    let da = (b.0 - a.0, b.1 - a.1);
    let db = (a.0 - b.0, a.1 - b.1);

    let mut p = (a.0 + da.0, a.1 + da.1); 
    while p.0 >= 0 && p.0 <= n && p.1 >= 0 && p.1 <= m {
        antinodes.push(p);
        p = (p.0 + da.0, p.1 + da.1); 

    }
    let mut p = (b.0 + db.0, b.1 + db.1); 
    while p.0 >= 0 && p.0 <= n && p.1 >= 0 && p.1 <= m {
        antinodes.push(p);
        p = (p.0 + db.0, p.1 + db.1); 

    }

    antinodes
}

pub fn part2(input: String) -> u64 {
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut m = 0;
    let mut n = 0;
    input.split('\n').filter(|line| !line.is_empty()).enumerate().for_each(|(i, line)| {
        n = n.max(i as i32);
        line.chars().enumerate().for_each(|(j, c)|{
            m = m.max(j as i32);
            if c != '.' {
                positions.entry(c).or_insert(vec![(i as i32, j as i32)]).push((i as i32, j as i32));
            }
        });
    });
    
        
    let mut unique_pos: HashSet<(i32, i32)> = HashSet::new();
    for (_, pos) in positions {
        for a in &pos {
            for b in &pos {
                if a == b {
                    continue;
                }
                let nodes = get_antinodes_p2(a, b, n, m);
                for node in nodes {
                    if node.0 >= 0 && node.0 <= n  && node.1 >= 0 && node.1 <= m {
                        unique_pos.insert(node);
                    } 
                }
            }
        }
    }

    unique_pos.len() as u64
}
