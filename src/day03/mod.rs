use std::num::ParseIntError;
use crate::utils::ParseError;
use std::str::FromStr;

fn binary_to_decimal(s: &Vec<char>) -> Result<usize, ParseIntError> {
    let binary: String = s.iter().collect();
    usize::from_str_radix(&binary, 2)
}

#[derive(Clone, Debug)]
pub struct Reading {
    binary: Vec<char>,
}

impl FromStr for Reading {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary = s.chars().collect::<Vec<_>>();
        Ok(Self { binary })
    }
}

impl Reading {
    fn get_bit(&self, pos: usize) -> Option<&char> {
        self.binary.get(pos)
    }

    fn len(&self) -> usize {
        self.binary.len()
    }

    fn decimal(&self) -> Result<usize, ParseIntError> {
        binary_to_decimal(&self.binary)
    }
}

#[derive(Clone)]
pub struct Report {
    readings: Vec<Reading>,
    ignored: Vec<usize>,
}

impl Report {
    fn most_common_bit(&self, pos: usize) -> char {
        let mut num0 = 0;
        let mut num1 = 0;

        for i in 0..self.readings.len() {
            if self.ignored.contains(&i) {
                continue;
            }

            if let Some(c) = self.readings[i].get_bit(pos) {
                if c == &'0' {
                    num0 += 1;
                } else {
                    num1 += 1;
                }
            }
        }

        if num0 > num1 {
            '0'
        } else {
            '1'
        }
    }

    fn least_common_bit(&self, pos: usize) -> char {
        let most_common = self.most_common_bit(pos);

        if most_common == '1' {
            '0'
        } else {
            '1'
        }
    }

    fn ignore(&mut self, pos: usize, c: char) {
        for i in 0..self.readings.len() {
            if self.ignored.contains(&i) {
                continue;
            }

            if let Some(q) = self.readings[i].get_bit(pos) {
                if q == &c {
                    self.ignored.push(i);
                }
            }
        }
    }

    fn len(&self) -> Result<usize, ParseError> {
        self.readings.iter().map(|r| r.len()).max().ok_or(ParseError::new("No input?"))
    }

    fn number_of_reports(&self) -> usize {
        self.readings.len() - self.ignored.len()
    }

    fn first(&self) -> Option<&Reading> {
        for i in 0..self.readings.len() {
            if !self.ignored.contains(&i) {
                return self.readings.get(i);
            }
        }

        None
    }

    fn gamma(&self) -> Result<usize, ParseError> {
        let len = self.len()?;

        let mut gamma = vec![];
        for i in 0..len {
            gamma.push(self.most_common_bit(i));
        }

        Ok(binary_to_decimal(&gamma)?)
    }

    fn epsilon(&self) -> Result<usize, ParseError> {
        let len = self.len()?;

        let mut epsilon = vec![];
        for i in 0..len {
            epsilon.push(self.least_common_bit(i));
        }

        Ok(binary_to_decimal(&epsilon)?)
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Report, ParseError> {
    let readings: Vec<Reading> = input
        .lines()
        .filter(|s| *s != "")
        .map(|s| Reading::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(Report { readings, ignored: vec![] })
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
        if oxygen_report.most_common_bit(i) == '1' {
            oxygen_report.ignore(i, '0');
        } else {
            oxygen_report.ignore(i, '1');
        }

        if oxygen_report.number_of_reports() <= 1 {
            break;
        }
    }
    let oxygen = oxygen_report.first().ok_or(ParseError::new("Could not generate oxygen report"))?.decimal()?;

    let mut co2scrub_report = report.clone();
    for i in 0..co2scrub_report.len()? {
        if co2scrub_report.most_common_bit(i) == '1' {
            co2scrub_report.ignore(i, '1');
        } else {
            co2scrub_report.ignore(i, '0');
        }

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

