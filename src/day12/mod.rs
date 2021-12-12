use std::collections::HashSet;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::utils::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cave {
    Big(Vec<char>),
    Small(Vec<char>),
    Start,
    End,
}

impl FromStr for Cave {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_lowercase = s.to_lowercase().as_str() == s;
        let ch = s.chars().collect::<Vec<_>>();

        let cave = match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => if is_lowercase { Cave::Small(ch) } else { Cave::Big(ch) }
        };

        Ok(cave)
    }
}

#[derive(Debug)]
pub struct Caves {
    paths: Vec<Vec<Cave>>,
}

impl FromStr for Caves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let paths = s.lines()
            .map(|l| l.split("-")
                .map(|v| Cave::from_str(v))
                .collect::<Result<Vec<_>, ParseError>>())
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Caves { paths })
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Caves, ParseError> {
    Caves::from_str(input)
}

pub fn travel(start: Cave, caves: &Caves, path: Vec<Cave>, visited: HashSet<Cave>) -> Vec<Vec<Cave>> {
    let mut visited = visited;
    let mut path = path;
    visited.insert(start.clone());
    path.push(start.clone());

    if start == Cave::End {
        return vec![path];
    }

    let mut results = vec![];
    let candidates = caves.paths.iter().filter(|p| p[0] == start);
    for c in candidates {
        let t = c[1].clone();
        if let Cave::Small(_) = t{
            if visited.contains(&t) {
                continue;
            }
        }

        if t == Cave::Start {
            continue;
        }

        results.append(&mut travel(c[1].clone(), caves, path.clone(), visited.clone()));
    }

    let candidates = caves.paths.iter().filter(|p| p[1] == start);
    for c in candidates {
        let t = c[0].clone();
        if let Cave::Small(_) = t{
            if visited.contains(&t) {
                continue;
            }
        }

        if t == Cave::Start {
            continue;
        }

        results.append(&mut travel(c[0].clone(), caves, path.clone(), visited.clone()));
    }

    results
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Caves) -> Result<usize, ParseError> {
    let valid_paths = travel(Cave::Start, input, vec![], HashSet::new());

    Ok(valid_paths.len())
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Caves) -> Result<i32, ParseError> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<i32>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
