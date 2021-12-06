use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input
        .split(",")
        .filter(|s| *s != "")
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

type Buckets = [usize; 9];

fn breed(buckets: &mut Buckets) {
    // This is basically a rotate_left but for better readability
    // with regards to accounting for the new born fish, we will
    // write it down explicitly.
    let breeding = buckets[0];
    for i in 0..8 {
        buckets[i] = buckets[i + 1];
    }
    buckets[6] += breeding;
    buckets[8] = breeding;
}

fn school_to_buckets(school: &Vec<u8>) -> Buckets {
    let mut bucket = [0; 9];

    for f in school {
        bucket[*f as usize] += 1;
    }

    bucket
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<u8>) -> Result<usize, ParseError> {
    let mut bucket = school_to_buckets(input);

    let days = 80;
    for _ in 0..days {
        breed(&mut bucket)
    }

    Ok(bucket.iter().sum())
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<u8>) -> Result<usize, ParseError> {
    let mut bucket = school_to_buckets(input);

    let days = 256;
    for _ in 0..days {
        breed(&mut bucket);
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

    fn input() -> Result<Vec<u8>, ParseError> {
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
