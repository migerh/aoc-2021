use std::collections::HashMap;
use std::num::ParseIntError;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<u8>>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.chars().map(|c| c.to_string().parse::<u8>()).collect::<Result<Vec<_>, ParseIntError>>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn build_map(input: &Vec<Vec<u8>>) -> HashMap<Coords, (u8, bool)> {
    input.iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter()
            .enumerate()
            .map(move |(x, v)| ((x as isize, y as isize), (*v, false)))
        })
        .flatten()
        .collect::<HashMap<_, _>>()
}

fn flash(x: isize, y: isize, map: &mut HashMap<Coords, (u8, bool)>) {
    let delta = vec![-1, 0, 1];

    for dy in &delta {
        for dx in &delta {
            if *dx == 0 && *dy == 0 {
                continue;
            }
            if let Some(v) = map.get_mut(&(x + dx, y + dy)) {
                v.0 += 1;

                if v.0 == 10 {
                    v.1 = true;
                    flash(x + dx, y + dy, map);
                }
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<u8>>) -> Result<i32, ParseError> {
    let sy = input.len() as isize;
    let sx = input[0].len() as isize;
    let mut map = build_map(input);
    let mut flash_count = 0;

    for _ in 0..100 {
        for y in 0..sy {
            for x in 0..sx {
                let v = map.get_mut(&(x, y)).ok_or(ParseError::new("Don't have entry"))?;
                v.0 += 1;

                if v.0 == 10 {
                    v.1 = true;
                    flash(x, y, &mut map);
                }
            }
        }

        for y in 0..sy {
            for x in 0..sx {
                let v = map.get_mut(&(x, y)).ok_or(ParseError::new("Don't have entry"))?;
                if v.1 {
                    v.0 = 0;
                    flash_count += 1;
                }
                v.1 = false;
            }
        }
    }

    Ok(flash_count)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<Vec<u8>>) -> Result<i32, ParseError> {
    let sy = input.len() as isize;
    let sx = input[0].len() as isize;
    let mut map = build_map(input);
    let mut flash_count = 0;
    let mut round = 0;
    let total = sx * sy;

    for i in 0..10000 {
        for y in 0..sy {
            for x in 0..sx {
                let v = map.get_mut(&(x, y)).ok_or(ParseError::new("Don't have entry"))?;
                v.0 += 1;

                if v.0 == 10 {
                    v.1 = true;
                    flash(x, y, &mut map);
                }
            }
        }

        let previous_flash = flash_count;

        for y in 0..sy {
            for x in 0..sx {
                let v = map.get_mut(&(x, y)).ok_or(ParseError::new("Don't have entry"))?;
                if v.1 {
                    v.0 = 0;
                    flash_count += 1;
                }
                v.1 = false;
            }
        }

        if flash_count - previous_flash == total {
            round = i+1;
            break;
        }
    }

    Ok(round)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    fn input() -> Result<Vec<Vec<u8>>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(1656, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(195, solve_part2(&data)?))
    }
}
