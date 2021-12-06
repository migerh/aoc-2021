use std::num::ParseIntError;
use crate::utils::ParseError;

#[derive(Debug, Clone)]
pub struct Fish {
    days: usize,
}

impl Fish {
    pub fn new(days: usize) -> Self {
        Fish { days }
    }

    pub fn tick(&mut self) -> bool {
        if self.days == 0 {
            self.days = 6;
            return true;
        }

        self.days -= 1;
        false
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Vec<Fish>, ParseIntError> {
    Ok(input
        .split(",")
        .filter(|s| *s != "")
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, ParseIntError>>()?
        .iter()
        .map(|f| Fish::new(*f))
        .collect())
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<Fish>) -> Result<usize, ParseError> {
    let mut fish = input.clone();
    let days = 80;
    let mut next;

    for _ in 0..days {
        next = vec![];
        for mut f in fish.into_iter() {
            if f.tick() {
                next.push(Fish::new(8))
            }
            next.push(f);
        }
        fish = next;
    }

    Ok(fish.len())
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Fish>) -> Result<usize, ParseError> {
    let mut bucket = vec![0; 9];

    for f in input {
        bucket[f.days] += 1;
    }

    let days = 256;

    for _ in 0..days {
        let temp = bucket[0];
        for i in 0..8 {
            bucket[i] = bucket[i + 1];
        }
        bucket[6] += temp;
        bucket[8] = temp;
    }

    Ok(bucket.iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "3,4,3,1,2"
    }

    fn input() -> Result<Vec<Fish>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let fish = input()?;
        Ok(assert_eq!(5934, solve_part1(&fish)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let fish = input()?;
        Ok(assert_eq!(26984457539, solve_part2(&fish)?))
    }
}
