use std::cmp::max;
use std::str::FromStr;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);

pub struct TargetArea {
    x: Coords,
    y: Coords,
}

impl FromStr for TargetArea {
    type Err = ParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(TargetArea { x: (269, 292), y: (-68, -44) })
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<TargetArea, ParseError> {
    Ok(TargetArea::from_str(input)?)
}

fn is_in_target(p: Coords, target: &TargetArea) -> bool {
    target.x.0 <= p.0 && p.0 <= target.x.1 &&
    target.y.0 <= p.1 && p.1 <= target.y.1
}

fn shoot(start_velocity: Coords, target: &TargetArea) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut v = start_velocity;
    let mut y_max = 0;
    let mut target_hit = false;

    while y > target.y.0 && x < target.x.1 {
        x += v.0;
        y += v.1;
        v.0 = max(0, v.0 - 1);
        v.1 -= 1;

        if v.0 == 0 && x < target.x.0 {
            break;
        }

        if y < target.y.0 && v.1 < 0 {
            break;
        }

        target_hit |= is_in_target((x, y), target);
        y_max = max(y, y_max);

        if target_hit {
            break;
        }
    }

    if target_hit {
        Some(y_max)
    } else {
        None
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &TargetArea) -> Result<isize, ParseError> {
    let target = input;
    let mut y_max = 0;

    // range choice is part educated guesses and part luck
    for y in target.y.0..500 {
        for x in 0..target.x.1 {
            if y == 0 && x == 0 {
                continue;
            }

            if let Some(new_y) = shoot((x, y), target) {
                y_max = max(y_max, new_y);
            }
        }
    }

    Ok(y_max)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &TargetArea) -> Result<i32, ParseError> {
    let target = input;
    let mut valid = 0;

    // range choice is part educated guesses and part luck
    for y in target.y.0..1000 {
        for x in 0..target.x.1*4 {
            if y == 0 && x == 0 {
                continue;
            }

            if let Some(new_y) = shoot((x, y), target) {
                valid += 1;
            }
        }
    }

    Ok(valid)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn input() -> Result<TargetArea, ParseError> {
        Ok(TargetArea { x: (20, 30), y: (-10, -5) })
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(45, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(112, solve_part2(&data)?))
    }
}
