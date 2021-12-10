use std::usize;

const INPUT: &'static str = include_str!("../inputs/3.txt");

pub fn run() {
    println!("day 3, output 1: {}", parse1(INPUT));
    println!("day 3, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let line_length = s.lines().next().unwrap().chars().count();
    let lines = s.lines();
    let median_number = lines.count() / 2;
    let mut counts = vec![0; line_length];
    for line in s.lines() {
        line.chars().enumerate().for_each(|(idx, val)| {
            if val == '1' {
                counts[idx] += 1
            }
        })
    }

    let gamma = counts
        .iter()
        .map(|count| if count >= &median_number { 1 } else { 0 })
        .fold(0, |acc, x| (acc << 1) | x);

    let epsilon = counts
        .iter()
        .map(|count| if count >= &median_number { 0 } else { 1 })
        .fold(0, |acc, x| (acc << 1) | x);

    gamma * epsilon
}

pub fn parse2(s: &str) -> usize {
    let lines: Vec<&str> = s.lines().into_iter().map(|line| line).collect();
    let byte_length: usize = lines[0].len();

    let oxygen: u32 = parse_gas(lines.clone(), 0, false, byte_length);
    let co2 = parse_gas(lines.clone(), 0, true, byte_length);

    (oxygen * co2) as usize
}

fn parse_gas(input: Vec<&str>, target_index: usize, invert: bool, byte_length: usize) -> u32 {
    if target_index < byte_length && input.len() > 1 {
        let (zeroes, ones): (Vec<&str>, Vec<&str>) = input
            .iter()
            .partition(|&line| -> bool { line.chars().nth(target_index).unwrap() == '0' });

        let next_iter = if (ones.len() >= zeroes.len()) ^ invert { ones } else { zeroes };

        return parse_gas(next_iter, target_index + 1, invert, byte_length);
    }
    return input[0]
        .chars()
        .map(|bit| -> u32 { bit.to_digit(10).unwrap() })
        .fold(0, |acc, number| (acc << 1) | number);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 198);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 230);
    }
}
