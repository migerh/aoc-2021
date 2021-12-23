use std::num::ParseIntError;
use crate::utils::ParseError;

#[derive(Debug, Clone)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
    None,
}

#[derive(Debug, Clone)]
pub struct Rooms {
    // This is how the hallway and the rooms
    // are stored in this structure:
    //
    // #############
    // #01.2.3.4.56#  <- hallway
    // ###0#1#2#3###  <- rooms
    //   #4#5#6#7#    <- rooms
    //   #########
    hallway: [char; 7],
    rooms: [char; 8],
}

#[aoc_generator(day23)]
pub fn input_generator(_input: &str) -> Result<Rooms, ParseIntError> {
    // let rooms = ['C', 'B', 'D', 'D', 'B', 'C', 'A', 'A'];

    let rooms = ['B', 'C', 'B', 'D', 'A', 'D', 'C', 'A'];
    let hallway = ['.', '.', '.', '.', '.', '.', '.'];
    Ok(Rooms { rooms, hallway })
}

impl Rooms {
    fn possible_movements(&self) -> Vec<(usize, usize)> {
        // get the top row out of their rooms, that are not in their
        // rooms yet or still have visitors.

        vec![]
    }
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Rooms) -> Result<i32, ParseError> {
    let rooms = input.clone();
    Ok(0)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Rooms) -> Result<i32, ParseError> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Rooms, ParseError> {
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
