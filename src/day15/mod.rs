use std::collections::HashSet;
use std::collections::HashMap;
use std::num::ParseIntError;
use crate::utils::ParseError;
use pathfinding::prelude::dijkstra;

type C = isize;
type Coords = (C, C);

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<u8>>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.chars().map(|c| c.to_string().parse::<u8>()).collect::<Result<Vec<_>, ParseIntError>>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn get_neighbors(p: &Coords, map: &Vec<Vec<u8>>) -> Vec<(Coords, usize)> {
    let delta = [-1, 1];
    let mut neighbors = vec![];

    for dy in &delta {
        let y = p.1 + dy;
        if y < 0 || y >= map.len() as isize {
            continue;
        }

        neighbors.push(((p.0, y), map[y as usize][p.0 as usize] as usize));
    }

    for dx in &delta {
        let x = p.0 + dx;
        if x < 0 || x >= map[0].len() as isize {
            continue;
        }

        neighbors.push(((x, p.1), map[p.1 as usize][x as usize] as usize));
    }

    neighbors
}

fn next(q: &HashSet<Coords>, dist: &HashMap<Coords, usize>) -> Option<Coords> {
    dist.iter()
        .filter(|(k, _)| !q.contains(*k))
        .max_by(|a, b| b.1.cmp(&a.1))
        .map(|(k, _)| *k)
}

fn my_dijkstra(start: &Coords, end: &Coords, map: &Vec<Vec<u8>>) -> Result<usize, ParseError> {
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();

    dist.entry(*start).or_insert(0);

    while let Some(u) = next(&visited, &dist) {
        visited.insert(u);

        let neighbors = get_neighbors(&u, map);
        for n in neighbors {
            let new_dist = dist.get(&u).ok_or(ParseError::new("Should not happen"))? + n.1;
            if let Some(&old_dist) = dist.get(&n.0) {
                if new_dist < old_dist {
                    dist.entry(n.0).and_modify(|v| *v = new_dist);
                }
            } else {
                dist.entry(n.0).or_insert(new_dist);
            }
        }
    }

    Ok(*dist.get(end).unwrap())
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Vec<Vec<u8>>) -> Result<usize, ParseError> {
    let start: Coords = (0, 0);
    let end: Coords = (input[0].len() as isize - 1, input.len() as isize - 1);
    let total_risk = my_dijkstra(&start, &end, input)?;

    Ok(total_risk)
}

fn expand(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let expansion1 = map.iter().map(|r| r.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>();
    let expansion2 = expansion1.iter().map(|r| r.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>();
    let expansion3 = expansion2.iter().map(|r| r.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>();
    let expansion4 = expansion3.iter().map(|r| r.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut result = map.clone();
    for i in 0..map.len() {
        result[i].append(&mut expansion1[i].clone());
        result[i].append(&mut expansion2[i].clone());
        result[i].append(&mut expansion3[i].clone());
        result[i].append(&mut expansion4[i].clone());
    }

    let mut expansiony = result.iter().map(|r| r.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>();
    for i in 0..4 {
        result.append(&mut expansiony);
        expansiony = result.iter().skip((i+1) * map.len()).map(|r| r.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>();
    }

    result
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<Vec<u8>>) -> Result<usize, ParseError> {
    let expansion = expand(input);

    let start: Coords = (0, 0);
    let end: Coords = (expansion[0].len() as isize - 1, expansion.len() as isize - 1);
    let result = dijkstra(&start, |p| get_neighbors(p, &expansion), |&p| p == end).ok_or(ParseError::new("Pathfinding error"))?;

    let total_risk = result.1;

    Ok(total_risk)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
    }

    fn input() -> Result<Vec<Vec<u8>>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(40, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(315, solve_part2(&data)?))
    }
}
