use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/6.txt");

pub fn run() {
    println!("day 6, output 1: {}", parse1(INPUT));
    println!("day 6, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
   let mut buckets = Bucket::from_str(s).unwrap();
   for _ in 0..80 {
       buckets.step();
   }
   return buckets.conut() as usize;
}

pub fn parse2(s: &str) -> usize {
    let mut buckets = Bucket::from_str(s).unwrap();
   for _ in 0..256 {
       buckets.step();
   }
   return buckets.conut() as usize;
}

struct Bucket {
    buckets: [u64; 9],
}

impl FromStr for Bucket {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buckets = [0; 9];
        s.trim().split(',')
            .map(|numb| -> usize { numb.parse().unwrap() })
            .for_each(|idx| buckets[idx] += 1);
        Ok(Bucket { buckets })
    }
}

impl Bucket {
    fn step(&mut self) {
        self.buckets.rotate_left(1);
        self.buckets[6] += self.buckets[8];
    }

    fn conut(&self) -> u64 {
        self.buckets.iter().fold(0, |acc: u64, count| acc + u64::from(*count))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 5934);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 26984457539);
    }
}
