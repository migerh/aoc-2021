use std::cmp::max;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::utils::ParseError;

type C = usize;
type Coords = (C, C);

#[derive(Debug)]
pub enum FoldInstruction {
    X(usize),
    Y(usize),
}

impl FromStr for FoldInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let instr = s.split(" ").skip(2).map(|f| f.split("=")).flatten().collect::<Vec<_>>();
        let axis = instr[0];
        let which = instr[1].parse::<usize>()?;

        Ok(match (axis, which) {
            ("y", w) => FoldInstruction::Y(w),
            ("x", w) => FoldInstruction::X(w),
            (_, _) => Err(ParseError::new("Unknown axis"))?,
        })
    }
}

#[derive(Debug)]
pub struct Paper {
    dots: Vec<Coords>,
    fold: Vec<FoldInstruction>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Paper, ParseError> {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    if parts.len() != 2 {
        return Err(ParseError::new("Broken input"));
    }

    let dots = parts[0]
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.split(",").map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, ParseIntError>>())
        .collect::<Result<Vec<_>, ParseIntError>>()?
        .into_iter()
        .map(|n| (n[0], n[1]))
        .collect::<Vec<_>>();
    let fold = parts[1]
        .lines()
        .filter(|s| *s != "")
        .map(|s| FoldInstruction::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(Paper { dots, fold })
}

fn build_map(input: &Paper) -> Vec<Vec<bool>> {
    let mut size = (0, 0);

    for d in &input.dots {
        size.0 = max(size.0, d.0);
        size.1 = max(size.1, d.1);
    }
    size = (size.0 + 1, size.1 + 1);

    let mut map = vec![vec![false; size.0]; size.1];

    for d in &input.dots {
        map[d.1][d.0] = true;
    }

    map
}

fn fold_x(input: &mut Vec<Vec<bool>>, w: usize) {
    for y in 0..input.len() {
        for x in w+1..input[y].len() {
            input[y][w+w-x] |= input[y][x];
        }
    }

    for y in 0..input.len() {
        input[y].resize(w, false);
    }
}

fn fold_y(input: &mut Vec<Vec<bool>>, w: usize) {
    for y in w+1..input.len() {
        for x in 0..input[y].len() {
            input[w+w-y][x] |= input[y][x];
        }
    }

    input.resize(w, vec![]);
}

fn print(input: &Vec<Vec<bool>>) {
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Paper) -> Result<usize, ParseError> {
    let mut map = build_map(input);

    let first = &input.fold[0];
    if let FoldInstruction::X(w) = first {
        fold_x(&mut map, *w);
    } else if let FoldInstruction::Y(w) = first {
        fold_y(&mut map, *w);
    }

    let result: usize = map.iter().map(|r| r.iter().filter(|c| **c).count()).sum();

    Ok(result)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Paper) -> Result<usize, ParseError> {
    let mut map = build_map(input);

    for fold in &input.fold {
        if let FoldInstruction::X(w) = fold {
            fold_x(&mut map, *w);
        } else if let FoldInstruction::Y(w) = fold {
            fold_y(&mut map, *w);
        }
    }

    print(&map);

    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
    }

    fn input() -> Result<Paper, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(17, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
