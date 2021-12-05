use num::signum;
use std::collections::HashMap;
use regex::Regex;
use crate::utils::ParseError;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coords {
    x: i32,
    y: i32,
}

impl FromStr for Coords {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .trim()
            .split(",")
            .map(|c| c.parse::<i32>())
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        if coords.len() != 2 {
            return Err(ParseError::new("Invalid number of coords"));
        }

        Ok(Self { x: coords[0], y: coords[1]})
    }
}

#[derive(Debug, Clone)]
pub struct Vent {
    start: Coords,
    end: Coords
}

impl FromStr for Vent {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(?P<start>.*)? -> (?P<end>.*)$").unwrap();
        }

        let (start, end) = RE.captures(s).and_then(|cap| {
            let start= cap.name("start").map(|v| v.as_str())?;
            let start = Coords::from_str(&start).ok()?;
            let end= cap.name("end").map(|v| v.as_str())?;
            let end = Coords::from_str(&end).ok()?;

            Some((start, end))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self { start, end })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Vec<Vent>, ParseError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| Vent::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn draw_vent(map: &mut HashMap<Coords, usize>, vent: &Vent) {
    let direction_x = signum(vent.end.x - vent.start.x);
    let direction_y = signum(vent.end.y - vent.start.y);

    let mut x = vent.start.x;
    let mut y = vent.start.y;
    while x != vent.end.x + direction_x || y != vent.end.y + direction_y {
        map.entry(Coords { x, y }).and_modify(|v| *v += 1).or_insert(1);
        x += direction_x;
        y += direction_y;
    }
}

fn draw_map(vents: &Vec<Vent>) -> HashMap<Coords, usize> {
    let mut map = HashMap::new();

    for v in vents {
        draw_vent(&mut map, v);
    }

    map
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<Vent>) -> Result<usize, std::num::ParseIntError> {
    let filtered = input.into_iter()
        .filter(|v| v.start.x == v.end.x || v.start.y == v.end.y)
        .cloned()
        .collect::<Vec<_>>();

    let map = draw_map(&filtered);
    let count = map.iter().filter(|(_k, v)| *v > &1).count();

    Ok(count)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<Vent>) -> Result<usize, std::num::ParseIntError> {
    let map = draw_map(&input);
    let count = map.iter().filter(|(_k, v)| *v > &1).count();

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
    }

    fn sample() -> Result<Vec<Vent>, ParseError> {
        input_generator(input())
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let sample = sample()?;
        assert_eq!(solve_part1(&sample)?, 5);
        Ok(())
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let sample = sample()?;
        assert_eq!(solve_part2(&sample)?, 12);
        Ok(())
    }
}
