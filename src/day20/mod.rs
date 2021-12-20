use std::cmp::max;
use std::num::ParseIntError;
use std::collections::HashMap;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);
type Map = HashMap<Coords, char>;

#[derive(Debug, Clone)]
pub struct Image {
    pixels: Map,
    outside: char,
    top_left: Coords,
    size: Coords,
}

impl Image {
    fn new(pixels: Map, size: Coords) -> Self {
        let outside = '.';
        let top_left = (0, 0);
        Image { pixels, outside, top_left, size }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<(Vec<char>, Image), ParseError> {
    let filter = input
        .lines()
        .take(1)
        .next().ok_or(ParseError::new("No image filter found"))?
        .chars()
        .collect::<Vec<_>>();

    let pixels = input
        .lines()
        .skip(2)
        .filter(|s| *s != "")
        .enumerate()
        .map(|(y, l)| l.chars()
            .enumerate()
            .map(move |(x, v)| {
                ((y as isize, x as isize), v)
            })
            .collect::<Vec<_>>())
        .flatten()
        .collect::<Map>();

    let mut size = (0, 0);
    for (i, _) in &pixels {
        size.0 = max(size.0, i.0);
        size.1 = max(size.1, i.1);
    }
    Ok((filter, Image::new(pixels, size) ))
}

impl Image {
    fn get_neighbors(&self, p: &Coords) -> Vec<char> {
        let mut result = vec![];
        let delta = vec![-1, 0, 1];

        for dy in &delta {
            for dx in &delta {
                let y = p.0 + dy;
                let x = p.1 + dx;

                result.push(*self.pixels.get(&(y, x)).unwrap_or(&self.outside));
            }
        }

        result
    }

    fn fold(&self, filter: &Vec<char>) -> Result<Self, ParseError> {
        let new_top_left = (self.top_left.0 - 1, self.top_left.1 - 1);
        let new_size = (self.size.0 + 2, self.size.1 + 2);
        let new_outside = match self.outside {
            '#' => filter[511],
            '.' => filter[0],
            _ => panic!("Cannot happen"),
        };

        let mut new_pixels = Map::new();

        for y in new_top_left.0..=new_size.0 {
            for x in new_top_left.1..=new_size.1 {
                let neighbors = self.get_neighbors(&(y, x));
                let index = binary_to_decimal(&neighbors)?;
                let new_c = filter[index];
                new_pixels.entry((y, x)).or_insert(new_c);
            }
        }

        Ok(Image { outside: new_outside, pixels: new_pixels, top_left: new_top_left, size: new_size })
    }

    fn count(&self) -> usize {
        self.pixels.iter().filter(|(_, v)| **v == '#').count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.top_left.0..=self.size.0 {
            for x in self.top_left.1..=self.size.1 {
                print!("{}", self.pixels.get(&(y, x)).unwrap_or(&'!'));
            }
            println!("");
        }
    }
}

fn binary_to_decimal(s: &Vec<char>) -> Result<usize, ParseIntError> {
    let binary: String = s.iter().map(|c| if *c == '#' { '1' } else { '0' }).collect();
    usize::from_str_radix(&binary, 2)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(Vec<char>, Image)) -> Result<usize, ParseError> {
    let filter = &input.0;
    let mut image = input.1.clone();

    for _ in 0..2 {
        image = image.fold(filter)?;
    }

    Ok(image.count())
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(Vec<char>, Image)) -> Result<usize, ParseError> {
    let filter = &input.0;
    let mut image = input.1.clone();

    for _ in 0..50 {
        image = image.fold(filter)?;
    }

    Ok(image.count())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
    }

    fn input() -> Result<(Vec<char>, Image), ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(35, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(3351, solve_part2(&data)?))
    }
}
