use std::collections::HashMap;
use std::num::ParseIntError;
use crate::utils::ParseError;

type C = i32;
type Coords = (C, C);
type Item = i32;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<Item>>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.chars().map(|c| c.to_string()).filter(|c| c != "\n").map(|c| c.parse::<Item>()).collect::<Result<Vec<_>, ParseIntError>>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn build_map(input: &Vec<Vec<i32>>) -> HashMap<Coords, Item> {
    let mut map = HashMap::new();

    input.iter()
        .enumerate()
        .for_each(|(y, r)| {
            r.iter()
            .enumerate()
            .for_each(|(x, v)| { map.entry((x as i32, y as i32)).or_insert(*v); })
        }
        );

    map
}

fn is_minimum(map: &HashMap<Coords, Item>, c: &Coords) -> bool {
    let delta: Vec<i32> = vec![-1, 1];

    if let Some(value) = map.get(c) {
        for x in &delta {
            if let Some(compare) = map.get(&(c.0 + x, c.1)) {
                if value >= compare {
                    return false;
                }
            }
        }

        for y in &delta {
            if let Some(compare) = map.get(&(c.0, c.1 + y)) {
                if value >= compare {
                    return false;
                }
            }
        }

        return true;
    }

    true
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> Result<Item, ParseError> {
    let map = build_map(input);

    let risk_levels = map.iter()
        .filter(|(c, _)| is_minimum(&map, *c))
        .map(|(_, v)| v + 1).sum();

    Ok(risk_levels)
}

fn basin_size(map: &HashMap<Coords, Item>, c: &Coords) -> usize {
    let mut queue = vec![*c];
    let mut visited = vec![];
    let mut basin = vec![];
    let delta = vec![-1, 1];

    while let Some(q) = queue.pop() {
        if visited.contains(&q) {
            continue;
        }
        visited.push(q);

        if let Some(v) = map.get(&q) {
            if *v != 9 {
                basin.push(*v);
            } else {
                continue;
            }

            for d in &delta {
                queue.push((q.0 + d, q.1));
                queue.push((q.0, q.1 + d));
            }
        }
    }

    basin.len()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<Item>>) -> Result<usize, ParseError> {
    let map = build_map(input);

    let minima = map.iter().filter(|(c, _)| is_minimum(&map, *c)).map(|(c, _)| c).collect::<Vec<_>>();

    let mut basin_sizes = minima.iter().map(|c| basin_size(&map, c)).collect::<Vec<_>>();
    basin_sizes.sort();
    let result = basin_sizes.iter().skip(basin_sizes.len() - 3).product();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "2199943210
3987894921
9856789892
8767896789
9899965678"
    }

    fn input() -> Result<Vec<Vec<i32>>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(15, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(1134, solve_part2(&data)?))
    }
}
