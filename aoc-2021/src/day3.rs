const INPUT: &'static str = include_str!("../inputs/3.txt");

pub fn run() {
    println!("day 3, output 1: {}", parse1(INPUT));
    println!("day 3, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
	let line_length =  s.lines().next().unwrap().chars().count();
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

pub fn parse2(_s: &str) -> usize {
    todo!()
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
        assert_eq!(parse2(INPUT), 0);
    }
}
