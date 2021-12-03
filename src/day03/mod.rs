use crate::day03::report::Report;
use crate::day03::reading::Reading;
use std::num::ParseIntError;
use crate::utils::ParseError;
use std::str::FromStr;

mod reading;
mod report;

fn binary_to_decimal(s: &Vec<char>) -> Result<usize, ParseIntError> {
    let binary: String = s.iter().collect();
    usize::from_str_radix(&binary, 2)
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Report, ParseError> {
    let readings: Vec<Reading> = input
        .lines()
        .filter(|s| *s != "")
        .map(|s| Reading::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(Report::new(readings))
}

#[aoc(day3, part1)]
pub fn solve_part1(report: &Report) -> Result<usize, ParseError> {
    let gamma = report.gamma()?;
    let epsilon = report.epsilon()?;

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
pub fn solve_part2(report: &Report) -> Result<usize, ParseError> {
    let mut oxygen_report = report.clone();

    for i in 0..oxygen_report.len()? {
        let lcb = oxygen_report.least_common_bit(i);
        oxygen_report.ignore(i, lcb);

        if oxygen_report.number_of_reports() <= 1 {
            break;
        }
    }
    let oxygen = oxygen_report.first().ok_or(ParseError::new("Could not generate oxygen report"))?.decimal()?;

    let mut co2scrub_report = report.clone();
    for i in 0..co2scrub_report.len()? {
        let mcb = co2scrub_report.most_common_bit(i);
        co2scrub_report.ignore(i, mcb);

        if co2scrub_report.number_of_reports() <= 1 {
            break;
        }
    }
    let co2scrub = co2scrub_report.first().ok_or(ParseError::new("Could not generate co2 scrub report"))?.decimal()?;

    Ok(oxygen * co2scrub)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "00100
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
01010"
    }

    fn input() -> Result<Report, ParseError> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let stuff = input()?;
        Ok(assert_eq!(198, solve_part1(&stuff)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let stuff = input()?;
        Ok(assert_eq!(230, solve_part2(&stuff)?))
    }
}

