use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .split(",")
        .filter(|s| *s != "")
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn fuel_consumption_id(distance: i32) -> i32 {
    distance
}

fn fuel_consumption_gauss(distance: i32) -> i32 {
    let n = distance;
    n * (n + 1) / 2
}

fn calculate_fuel(crabs: &Vec<i32>, alignment: i32, fuel_consumption: fn(i32) -> i32) -> i32 {
    crabs.iter().map(|c| fuel_consumption((c - alignment).abs())).sum()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<i32>) -> Result<i32, ParseError> {
    let max = *input.iter().max().ok_or(ParseError::new("There are no crabs?"))?;

    let min_fuel = (0..=max)
        .map(|alignment| calculate_fuel(input, alignment, fuel_consumption_id))
        .min()
        .ok_or(ParseError::new("No fuel?"))?;

    Ok(min_fuel)
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<i32>) -> Result<i32, ParseError> {
    let max = *input.iter().max().ok_or(ParseError::new("There are no crabs?"))?;

    let min_fuel = (0..=max)
        .map(|alignment| calculate_fuel(input, alignment, fuel_consumption_gauss))
        .min()
        .ok_or(ParseError::new("No fuel?"))?;

    Ok(min_fuel)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    fn input() -> Result<Vec<i32>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(37, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(168, solve_part2(&data)?))
    }
}
