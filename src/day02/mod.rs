use crate::utils::ParseError;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Command {
    dir: Direction,
    len: i32,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(.*)? (\d)$").unwrap();
        }

        let cap = RE.captures(s).unwrap();
        let dir_str = cap[1].to_lowercase();
        let len = cap[2].parse::<i32>()?;

        let dir = if dir_str == "forward" {
            Direction::Forward
        } else if dir_str == "up" {
            Direction::Up
        } else {
            Direction::Down
        };

        Ok(Self { dir, len })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Command>, ParseError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| Command::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Command>) -> Result<i32, ParseError> {
    let (horizontal, depth) = input.into_iter().fold((0, 0), |acc, c| {
        match (c.dir, c.len) {
            (Direction::Forward, v) => (acc.0 + v, acc.1),
            (Direction::Up, v) => (acc.0, acc.1 - v),
            (Direction::Down, v) => (acc.0, acc.1 + v),
        }
    });

    Ok(horizontal * depth)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Command>) -> Result<i32, std::num::ParseIntError> {
    let (horizontal, depth, _aim) = input.into_iter().fold((0, 0, 0), |acc, c| {
        match (c.dir, c.len) {
            (Direction::Forward, v) => (acc.0 + v, acc.1 + v * acc.2, acc.2),
            (Direction::Up, v) => (acc.0, acc.1, acc.2 - v),
            (Direction::Down, v) => (acc.0, acc.1, acc.2 + v),
        }
    });

    Ok(horizontal * depth)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "forward 5
down 5
forward 8
up 3
down 8
forward 2"
    }

    fn input() -> Result<Vec<Command>, ParseError> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let commands = input()?;
        Ok(assert_eq!(150, solve_part1(&commands)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let commands = input()?;
        Ok(assert_eq!(900, solve_part2(&commands)?))
    }
}
