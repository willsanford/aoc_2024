pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

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
        (5, 1) => day5::part1(input),
        (5, 2) => day5::part2(input),
        (6, 1) => day6::part1(input),
        (6, 2) => day6::part2(input),
        (7, 1) => day7::part1(input),
        (7, 2) => day7::part2(input),
        (8, 1) => day8::part1(input),
        (8, 2) => day8::part2(input),
        _ => {
            eprintln!("Not implemented for day {} part {}", day, part);
            0
        }
    }
}
