const INPUT: &'static str = include_str!("../inputs/1.txt");

pub fn run() {
    println!("day 1, output 1: {}", parse1(INPUT));
    println!("day 1, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let inputs: Vec<u16> = s.lines().map(|line| line.parse().unwrap()).collect();
    let count = inputs.array_windows().filter(|[n1, n2]| n1 < n2).count();
    count
}

pub fn parse2(s: &str) -> usize {
    let inputs: Vec<u16> = s.lines().map(|line| line.parse().unwrap()).collect();
    let count = inputs.array_windows().filter(|[n1, n2, n3, n4]| n1 + n2 + n3 < n2 + n3 + n4).count();
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 7);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 5);
    }
}