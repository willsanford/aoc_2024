fn check(target: u64, current: u64, vals: &Vec<u64>) -> bool {
    if vals.is_empty() {
        return target == current;
    }

    let mut new_vals = vals.clone();
    let x = new_vals.pop().unwrap();

    if current == 0 {
        return check(target, x, &new_vals);        
    }

    return check(target, current + x, &new_vals) || check(target, current * x, &new_vals);
}


pub fn part1(input: String) -> u64 {
    let data: Vec<(u64, Vec<u64>)> = input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (tot, vals) = line.split_once(':').unwrap();

            let parsed_vals = vals.split(' ')
                                  .filter(|val| !val.is_empty())
                                  .map(|val|{ val.parse::<u64>().unwrap()})
                                  .rev().collect::<Vec<u64>>();
        (tot.parse::<u64>().unwrap(), parsed_vals)
        }).collect();

    data.iter().filter(|(target, vals)| {
        check(*target, 0, vals)
    }).map(|(target, _)|{target}).sum()
}

fn concat_u64(a: u64, b: u64) -> u64 { 
    a * 10_u64.pow(b.to_string().len() as u32) + b
} 


fn check_p2(target: u64, current: u64, vals: &Vec<u64>) -> bool {
    if vals.is_empty() {
        return target == current;
    }

    let mut new_vals = vals.clone();
    let x = new_vals.pop().unwrap();

    if current == 0 {
        return check_p2(target, x, &new_vals);        
    }

    return check_p2(target, current + x, &new_vals) 
        || check_p2(target, current * x, &new_vals) 
        || check_p2(target, concat_u64(current, x), &new_vals);
}

pub fn part2(input: String) -> u64 {
    let data: Vec<(u64, Vec<u64>)> = input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (tot, vals) = line.split_once(':').unwrap();

            let parsed_vals = vals.split(' ')
                                  .filter(|val| !val.is_empty())
                                  .map(|val|{ val.parse::<u64>().unwrap()})
                                  .rev().collect::<Vec<u64>>();
        (tot.parse::<u64>().unwrap(), parsed_vals)
        }).collect();

    data.iter().filter(|(target, vals)| {
        check_p2(*target, 0, vals)
    }).map(|(target, _)|{target}).sum()
}
