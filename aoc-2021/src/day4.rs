use std::{collections::HashMap, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/4.txt");

pub fn run() {
    println!("day 4, output 1: {}", parse1(INPUT));
    println!("day 4, output 2: {}", parse2(INPUT));
}


pub fn parse1(s: &str) -> usize {
   let (numbers, mut boards) = parse_input(s);
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.is_winner() {
                return (board.get_unmarked_numbers().iter().sum::<u64>() * number) as usize;
            }
        }
    }

    0
    

}

pub fn parse2(s: &str) -> usize {
    let (numbers, mut boards) = parse_input(s);

    let mut boards_left = boards.len();

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.is_winner() && boards_left == 1{
                return (board.get_unmarked_numbers().iter().sum::<u64>() * number) as usize;
            }
        }

        boards = boards.into_iter().filter_map(|board| match board.is_winner() {
            false => Some(board.clone()),
            true => None
        }).collect();
            
        boards_left = boards.len();
    }

    0

}

fn parse_input (s: &str) -> (Vec<u64>, Vec<Board>) {
    let mut items = s.split("\n\n").into_iter();

    let numbers = items.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();

    let boards = items.peekable().map(|s| s.to_owned().parse().unwrap()).collect();

    return (numbers, boards);
}


#[derive(Debug, Clone)]
pub struct Board {
    pub numbers: HashMap<(usize, usize), u64>,
    pub marks: HashMap<(usize, usize), bool>,
}
 
impl FromStr for Board {
    type Err = std::io::Error;
 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = HashMap::new();
        let mut marks = HashMap::new();
 
        for (y, line) in s.trim().lines().enumerate() {
            for (x, n) in line.split_whitespace().enumerate() {
                numbers.insert((x, y), n.parse().unwrap());
                marks.insert((x, y), false);
            }
        }
 
        Ok(Self { numbers, marks })
    }
}

impl Board {
    fn mark(&mut self, number: u64) {
        for (k,v) in &self.numbers {
            if v == &number {
                *self.marks.get_mut(k).unwrap() = true
            }
        }
    }

    fn is_winner(&self) -> bool {
        // Check x
        for i in 0..5 {
            let winner = self.marks.iter().filter(|((x, _), _)| x == &i).all(|((_, _), b)| *b);
            if winner { return true; }
        }

        // Check y
        for i in 0..5 {
            let winner = self.marks.iter().filter(|((_, y), _)| y == &i).all(|((_,_), b)| *b);
            if winner { return true; }
        }

        false
    }

    fn get_unmarked_numbers(&self) -> Vec<u64> {
        self.marks.iter().filter_map(|((x,y), b)| {
            match b {
                false => Some(*self.numbers.get(&(*x,*y)).unwrap()),
                true => None
            }
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 4512);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1924);
    }
}