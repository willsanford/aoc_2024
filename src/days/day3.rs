#[derive(PartialEq, PartialOrd, Debug)]
enum State {
    M,
    U,
    L,
    FirstParen,
    Digits1(Vec<u64>),
    Digits2(Vec<u64>),
    O,
    N,
    Appos,
    T,
    OpenParenDont,
    CloseParenDo,
    CloseParenDont,
}

fn get_number(input: Vec<u64>) -> u64 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, val)| 10_u64.pow(i as u32) * val)
        .sum()
}

fn parse_line(input: &str) -> Vec<(u64, u64)> {
    use State::*;
    let mut vals = Vec::new();
    let mut state = M;
    let mut current_digits: (u64, u64) = (0, 0);

    for char in input.chars() {
        state = match (state, char) {
            (M, 'm') => U,
            (U, 'u') => L,
            (L, 'l') => FirstParen,
            (FirstParen, '(') => Digits1(Vec::new()),
            (Digits1(d), ',') => {
                current_digits.0 = get_number(d);
                Digits2(Vec::new())
            }
            (Digits1(mut d), c) => {
                if c.is_numeric() {
                    d.push(c.to_digit(10).unwrap() as u64);
                    Digits1(d)
                } else {
                    M
                }
            }
            (Digits2(d), ')') => {
                current_digits.1 = get_number(d);
                vals.push(current_digits);
                M
            }
            (Digits2(mut d), c) => {
                if c.is_numeric() {
                    d.push(c.to_digit(10).unwrap() as u64);
                    Digits2(d)
                } else {
                    M
                }
            }
            _ => M,
        }
    }
    vals
}
pub fn part1(input: String) -> u64 {
    let vals = parse_line(&input);
    vals.iter().map(|(a, b)| a * b).sum()
}

fn parse_line_part2(input: &str) -> Vec<(u64, u64)> {
    use State::*;
    let mut vals = Vec::new();
    let mut should_add: bool = true;
    let mut state = M;
    let mut current_digits: (u64, u64) = (0, 0);

    for char in input.chars() {
        state = match (state, char) {
            (M, 'm') => U,
            (M, 'd') => O,
            (O, 'o') => N,
            (N, '(') => CloseParenDo,
            (CloseParenDo, ')') => {
                should_add = true;
                M
            }
            (N, 'n') => Appos,
            (Appos, '\'') => T,
            (T, 't') => OpenParenDont,
            (OpenParenDont, '(') => CloseParenDont,
            (CloseParenDont, ')') => {
                should_add = false;
                M
            }
            (U, 'u') => L,
            (L, 'l') => FirstParen,
            (FirstParen, '(') => Digits1(Vec::new()),
            (Digits1(d), ',') => {
                current_digits.0 = get_number(d);
                Digits2(Vec::new())
            }
            (Digits1(mut d), c) => {
                if c.is_numeric() {
                    d.push(c.to_digit(10).unwrap() as u64);
                    Digits1(d)
                } else {
                    M
                }
            }
            (Digits2(d), ')') => {
                current_digits.1 = get_number(d);
                if should_add {
                    vals.push(current_digits);
                }
                M
            }
            (Digits2(mut d), c) => {
                if c.is_numeric() {
                    d.push(c.to_digit(10).unwrap() as u64);
                    Digits2(d)
                } else {
                    M
                }
            }
            _ => M,
        }
    }
    vals
}
pub fn part2(input: String) -> u64 {
    let vals = parse_line_part2(&input);
    vals.iter().map(|(a, b)| a * b).sum()
}
