use crate::utils::ParseError;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>, ParseError> {
    Ok(input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

#[derive(Debug, Clone)]
pub struct Seafloor {
    floor: Vec<Vec<char>>,
}

impl Seafloor {
    pub fn new(floor: Vec<Vec<char>>) -> Self {
        Self { floor }
    }

    pub fn move_east(&self) -> Self {
        let mut new_floor = vec![];
        for y in 0..self.floor.len() {
            let mut new_row = vec!['.'; self.floor[0].len()];

            for x in 0..self.floor[0].len() {
                let nx = (x + 1) % self.floor[0].len();

                if self.floor[y][x] == '>' && self.floor[y][nx] == '.' {
                    new_row[nx] = '>';
                } else if self.floor[y][x] != '.' {
                    new_row[x] = self.floor[y][x];
                }
            }
            new_floor.push(new_row);
        }

        Self { floor: new_floor }
    }

    pub fn move_south(&self, east: Vec<Vec<char>>) -> Self {
        let mut new_floor = vec![vec!['.'; self.floor[0].len()]; self.floor.len()];
        for y in 0..east.len() {
            let ny = (y + 1) % east.len();

            for x in 0..east[0].len() {
                if east[y][x] == 'v' && east[ny][x] == '.' {
                    new_floor[ny][x] = 'v';
                } else if east[y][x] != '.' {
                    new_floor[y][x] = east[y][x];
                }
            }
        }

        Self { floor: new_floor }
    }

    pub fn tick(&self) -> Self {
        let east = self.move_east();
        self.move_south(east.floor)
    }

    pub fn print(&self) {
        for r in &self.floor {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
    }

    pub fn equal(&self, other: &Seafloor) -> bool {
        for y in 0..self.floor.len() {
            for x in 0..self.floor[0].len() {
                if self.floor[y][x] != other.floor[y][x] {
                    return false;
                }
            }
        }

        return true;
    }
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> Result<usize, ParseError> {
    let floor = Seafloor::new(input.clone());

    let mut current = floor;
    let mut stable = 0;
    loop {
        let next = current.tick();
        stable += 1;
        if next.equal(&current) {
            break;
        }
        current = next;
    }

    Ok(stable)
}

#[aoc(day25, part2)]
pub fn solve_part2(_input: &Vec<Vec<char>>) -> Result<usize, ParseError> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
    }

    fn input() -> Result<Vec<Vec<char>>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(58, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
