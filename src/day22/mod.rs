use crate::utils::ParseError;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

type C = isize;
type Range = (C, C);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cuboid {
    state: State,
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

impl FromStr for Cuboid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<state>.*) x=(?P<x_r_s>.*)?\.\.(?P<x_r_e>.*),y=(?P<y_r_s>.*)\.\.(?P<y_r_e>.*),z=(?P<z_r_s>.*)\.\.(?P<z_r_e>.*)$").unwrap();
        }

        let (state, x_range, y_range, z_range) = RE
            .captures(s)
            .and_then(|cap| {
                let state = cap.name("state").map(|v| v.as_str())?;
                let x_r_s = cap
                    .name("x_r_s")
                    .map(|v| v.as_str().parse::<isize>())?
                    .unwrap();
                let x_r_e = cap
                    .name("x_r_e")
                    .map(|v| v.as_str().parse::<isize>())?
                    .unwrap();
                let y_r_s = cap
                    .name("y_r_s")
                    .map(|v| v.as_str().parse::<isize>())?
                    .unwrap();
                let y_r_e = cap
                    .name("y_r_e")
                    .map(|v| v.as_str().parse::<isize>())?
                    .unwrap();
                let z_r_s = cap
                    .name("z_r_s")
                    .map(|v| v.as_str().parse::<isize>())?
                    .unwrap();
                let z_r_e = cap
                    .name("z_r_e")
                    .map(|v| v.as_str().parse::<isize>())?
                    .unwrap();

                Some((state, (x_r_s, x_r_e), (y_r_s, y_r_e), (z_r_s, z_r_e)))
            })
            .ok_or(ParseError::new("Error during parse"))?;

        let state = match state {
            "on" => State::On,
            "off" => State::Off,
            _ => Err(ParseError::new("Unknown state"))?,
        };

        Ok(Self {
            state,
            x_range,
            y_range,
            z_range,
        })
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Result<Vec<Cuboid>, ParseError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| Cuboid::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<Cuboid>) -> Result<usize, ParseError> {
    let mut map = HashMap::new();

    for c in input {
        for x in c.x_range.0..=c.x_range.1 {
            if x < -50 || x > 50 {
                continue;
            }
            for y in c.y_range.0..=c.y_range.1 {
                if y < -50 || y > 50 {
                    continue;
                }
                for z in c.z_range.0..=c.z_range.1 {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                        map.entry((x, y, z))
                            .and_modify(|v| *v = c.state.clone())
                            .or_insert(c.state.clone());
                    }
                }
            }
        }
    }

    let ons = map.iter().filter(|(_, v)| **v == State::On).count();

    Ok(ons)
}

impl Cuboid {
    fn has_intersection(&self, c2: &Cuboid) -> bool {
        let c1 = self;
        if c1.x_range.1 < c2.x_range.0 || c2.x_range.1 < c1.x_range.0 {
            return false;
        }

        if c1.y_range.1 < c2.y_range.0 || c2.y_range.1 < c1.y_range.0 {
            return false;
        }

        if c1.z_range.1 < c2.z_range.0 || c2.z_range.1 < c1.z_range.0 {
            return false;
        }

        true
    }

    fn intersect(&self, c2: &Cuboid) -> Cuboid {
        let c1 = self;
        let mut xs = vec![c1.x_range.0, c1.x_range.1, c2.x_range.0, c2.x_range.1];
        let mut ys = vec![c1.y_range.0, c1.y_range.1, c2.y_range.0, c2.y_range.1];
        let mut zs = vec![c1.z_range.0, c1.z_range.1, c2.z_range.0, c2.z_range.1];

        xs.sort();
        ys.sort();
        zs.sort();

        let state = self.state;
        let x_range = (xs[1], xs[2]);
        let y_range = (ys[1], ys[2]);
        let z_range = (zs[1], zs[2]);

        Cuboid {
            state,
            x_range,
            y_range,
            z_range,
        }
    }

    fn split(&self, splitter: &Cuboid) -> Vec<Cuboid> {
        if !self.has_intersection(splitter) {
            return vec![];
        }

        if self == splitter {
            return vec![];
        }

        //     +-----+--+
        //    /     /| /|
        //   +-----+--+-+
        //   |     |/ |/|
        //   |     +--+ |
        //   |        |/
        //   +--------+
        //

        let new_ranges = vec![
            // get the front cuboid
            (self.x_range, self.y_range, (self.z_range.0, splitter.z_range.0 - 1)),
            (self.x_range, self.y_range, (splitter.z_range.1 + 1, self.z_range.1)),
            // get the left cuboid
            ((self.x_range.0, splitter.x_range.0 - 1), self.y_range, splitter.z_range),
            ((splitter.x_range.1 + 1, self.x_range.1), self.y_range, splitter.z_range),
            // get the bottom cuboid
            (splitter.x_range, (self.y_range.0, splitter.y_range.0 - 1), splitter.z_range),
            (splitter.x_range, (splitter.y_range.1 + 1, self.y_range.1), splitter.z_range),
        ];

        // println!("self {:?}\nsplitter {:?}", self, splitter);

        let mut result = vec![];
        let state = self.state;
        for (x_range, y_range, z_range) in new_ranges {
            if x_range.0 <= x_range.1 && y_range.0 <= y_range.1 && z_range.0 <= z_range.1 {
                let c = Cuboid { state, x_range, y_range, z_range };
                result.push(c);
            }
        }

        // println!("split result: {:?}", result);

        result
    }

    fn size(&self) -> usize {
        let x = (self.x_range.1 - self.x_range.0 + 1).abs() as usize;
        let y = (self.y_range.1 - self.y_range.0 + 1).abs() as usize;
        let z = (self.z_range.1 - self.z_range.0 + 1).abs() as usize;

        x * y * z
    }

    #[allow(dead_code)]
    fn print(&self) {
        print!("([{}, {}], [{}, {}], [{}, {}])", self.x_range.0, self.x_range.1, self.y_range.0, self.y_range.1, self.z_range.0, self.z_range.1);
    }
}

pub struct CuboidMerge {
    area: Vec<Cuboid>,
}

impl CuboidMerge {
    fn new() -> Self {
        let area = vec![];
        CuboidMerge { area }
    }

    fn merge(&mut self, c: &Cuboid) {
        let mut intersects = vec![];
        let mut replacements = vec![];
        // println!("new cuboid {:?}", c);
        for i in 0..self.area.len() {
            if self.area[i].has_intersection(c) {
                intersects.push(i);
                let icube = self.area[i].intersect(c);
                // println!("int: {:?}", icube);
                // println!("cube {:?}\nintersection: {:?}", self.area[i], icube);
                replacements.push(self.area[i].split(&icube));

                // println!("");
            }
        }

        for i in intersects.into_iter().rev() {
            self.area.swap_remove(i);
        }

        for mut r in replacements.into_iter() {
            self.area.append(&mut r);
        }

        if c.state == State::On {
            self.area.push(c.clone());
        }

        // println!("#cuboids: {}", self.area.len());
    }

    fn count(&self) -> usize {
        self.area.iter().map(|c| c.size()).sum()
    }
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<Cuboid>) -> Result<usize, ParseError> {
    let mut area = CuboidMerge::new();

    for c in input {
        area.merge(c);
        // println!("\n\nCubes:");
        // for a in &area.area {
        //     a.print();
        // }
        // println!("\n\n");
    }

    Ok(area.count())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Cuboid>, ParseError> {
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
