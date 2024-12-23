pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day22;
pub mod day23;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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
        (9, 1) => day9::part1(input),
        (9, 2) => day9::part2(input),
        (10, 1) => day10::part1(input),
        (10, 2) => day10::part2(input),
        (11, 1) => day11::part1(input),
        (11, 2) => day11::part2(input),
        (12, 1) => day12::part1(input),
        (12, 2) => day12::part2(input),
        (13, 1) => day13::part1(input),
        (13, 2) => day13::part2(input),
        (14, 1) => day14::part1(input),
        (14, 2) => day14::part2(input),
        (17, 1) => day17::part1(input),
        (17, 2) => day17::part2(input),
        (18, 1) => day18::part1(input),
        (18, 2) => day18::part2(input),
        (19, 1) => day19::part1(input),
        (19, 2) => day19::part2(input),
        (20, 1) => day20::part1(input),
        (20, 2) => day20::part2(input),
        (22, 1) => day22::part1(input),
        (22, 2) => day22::part2(input),
        (23, 1) => day23::part1(input),
        (23, 2) => day23::part2(input),
        _ => {
            eprintln!("Not implemented for day {} part {}", day, part);
            0
        }
    }
}
