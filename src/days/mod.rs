pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn run_day(day: i16, part: i16, input: String) -> u64 {
    // TODO : do something interesting with days and macros
    match (day, part) {
        (1, 1) => day1::part1(input),
        (1, 2) => day1::part2(input),
        (2, 1) => day2::part1(input),
        (2, 2) => day2::part2(input),
        (3, 1) => day3::part1(input),
        (3, 2) => day3::part2(input),
        (4, 1) => day4::part1(input),
        (4, 2) => day4::part2(input),
        _ => {
            eprintln!("Not implemented for day {} part {}", day, part);
            0
        }
    }
}
