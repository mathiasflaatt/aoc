const INPUT: &'static str = include_str!("../inputs/2.txt");

pub fn run() {
    println!("day 2, output 1: {}", parse1(INPUT));
    println!("day 2, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;
    for line in s.lines() {
        let cmd: Command = line.parse().unwrap();
        match cmd {
            Command::Forward(x) => pos_x += x,
            Command::Up(y) => pos_y -= y,
            Command::Down(y) => pos_y += y,
        }
    }

    pos_x * pos_y
}
pub fn parse2(s: &str) -> usize {
    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;
    let mut aim: usize = 0;
    for line in s.lines() {
        let cmd: Command = line.parse().unwrap();
        match cmd {
            Command::Forward(x) => {
                pos_x += x;
                pos_y += aim * x;
            }
            Command::Up(y) => {
                aim -= y
            }
            Command::Down(y) => {
                aim += y
            }
        }
    }

    pos_x * pos_y
}

enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl std::str::FromStr for Command {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ').unwrap() {
            ("forward", x) => Ok(Command::Forward(x.parse().unwrap())),
            ("up", x) => Ok(Command::Up(x.parse().unwrap())),
            ("down", x) => Ok(Command::Down(x.parse().unwrap())),
            _ => panic!("Command not implemented"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 150);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 900);
    }
}
