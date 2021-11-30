use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> Result<u32, std::num::ParseIntError> {
    Ok(0)
}