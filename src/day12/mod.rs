use std::collections::HashMap;
use std::str::FromStr;
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

pub fn one_small_visited_twice(visited: &HashMap<Cave, usize>) -> bool {
    let number_small_ones_visited_twice = visited.iter().filter(|(k, v)| {
        if let Cave::Small(_) = k {
            if **v == 2 {
                true
            } else {
                false
            }
        } else {
            false
        }
    })
    .count();

    number_small_ones_visited_twice > 0
}

pub fn travel(start: Cave, caves: &Caves, path: Vec<Cave>, visited: HashMap<Cave, usize>, part2: bool) -> Vec<Vec<Cave>> {
    let mut visited = visited;
    let mut path = path;
    visited.entry(start.clone()).and_modify(|v| *v += 1).or_insert(1);
    path.push(start.clone());

    if start == Cave::End {
        return vec![path];
    }

    let mut results = vec![];
    let direction = vec![(0, 1), (1, 0)];
    for d in direction {
        let candidates = caves.paths.iter().filter(|p| p[d.0] == start);
        for c in candidates {
            let t = c[d.1].clone();
            if let Cave::Small(_) = t {
                if visited.contains_key(&t) {
                    if !part2 || one_small_visited_twice(&visited) {
                        continue;
                    }
                }
            }

            if t == Cave::Start {
                continue;
            }

            results.append(&mut travel(c[d.1].clone(), caves, path.clone(), visited.clone(), part2));
        }
    }

    results
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Caves) -> Result<usize, ParseError> {
    let valid_paths = travel(Cave::Start, input, vec![], HashMap::new(), false);
    Ok(valid_paths.len())
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Caves) -> Result<usize, ParseError> {
    let valid_paths = travel(Cave::Start, input, vec![], HashMap::new(), true);
    Ok(valid_paths.len())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample1() -> &'static str {
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end
dc-end"
    }

    fn input1() -> Result<Caves, ParseError> {
        Ok(input_generator(sample1())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input1()?;
        Ok(assert_eq!(10, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input1()?;
        Ok(assert_eq!(36, solve_part2(&data)?))
    }

    fn sample2() -> &'static str {
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
    }

    fn input2() -> Result<Caves, ParseError> {
        Ok(input_generator(sample2())?)
    }

    #[test]
    fn part1_sample2() -> Result<(), ParseError> {
        let data = input2()?;
        Ok(assert_eq!(19, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample2() -> Result<(), ParseError> {
        let data = input2()?;
        Ok(assert_eq!(103, solve_part2(&data)?))
    }
}
