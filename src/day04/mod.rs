use std::num::ParseIntError;
use crate::utils::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct BingoBoard {
    numbers: Vec<Vec<u32>>,
    marked: Vec<Vec<u32>>,
}

impl FromStr for BingoBoard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.lines()
            .filter(|v| *v != "")
            .map(|v| v
                .split(" ")
                .filter(|v| *v != "")
                .map(|w| w.parse::<u32>())
                .collect::<Result<Vec<_>, ParseIntError>>())
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        let height = numbers.len();
        if height != 5 {
            return Err(ParseError::new("Invalid height"));
        }

        let number_of_invalid_widths = numbers
            .iter()
            .map(|v| v.len())
            .filter(|v| *v != 5)
            .count();
        if number_of_invalid_widths > 0 {
            return Err(ParseError::new("Invalid width"));
        }

        let marked = vec![vec![0; 5]; 5];

        Ok( Self { numbers, marked })
    }
}

impl BingoBoard {
    fn mark(&mut self, n: u32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == n {
                    self.marked[i][j] = 1;
                }
            }
        }
    }

    fn done(&self) -> bool {
        let mut sum_rows = vec![0; 5];
        let mut sum_cols = vec![0; 5];
        let mut diag_1 = 0;
        let mut diag_2 = 0;

        for i in 0..self.marked.len() {
            for j in 0..self.marked[i].len() {
                sum_rows[i] += self.marked[i][j];
                sum_cols[j] += self.marked[i][j];

                if i == j {
                    diag_1 += self.marked[i][j];
                }

                if i == 5 - j {
                    diag_2 += self.marked[i][j];
                }
            }
        }

        if diag_1 == 5 || diag_2 == 5 {
            return true;
        }

        if sum_rows.contains(&5) {
            return true;
        }

        if sum_cols.contains(&5) {
            return true;
        }

        false
    }

    fn unmarked(&self) -> u32 {
        let mut unmarked_sum = 0;

        for i in 0..self.marked.len() {
            for j in 0..self.marked[i].len() {
                if self.marked[i][j] == 0 {
                    unmarked_sum += self.numbers[i][j];
                }
            }
        }

        unmarked_sum
    }
}

#[derive(Debug)]
pub struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl FromStr for BingoGame {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game = s
            .split("\n\n")
            .filter(|v| *v != "")
            .collect::<Vec<_>>();

        let numbers = game[0]
            .split(",")
            .map(|v| v.parse::<u32>())
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        let boards = game
            .iter()
            .skip(1)
            .map(|v| BingoBoard::from_str(v))
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { numbers, boards })
    }
}



#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Result<BingoGame, ParseError> {
    BingoGame::from_str(input)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &BingoGame) -> Result<u32, ParseError> {
    let game = input.numbers.clone();
    let mut boards = input.boards.clone();

    for n in &game {
        for i in 0..boards.len() {
            boards[i].mark(*n);

            if boards[i].done() {
                let unmarked = boards[i].unmarked();
                return Ok(unmarked * n);
            }
        }
    }

    Err(ParseError::new("Not all boards are completed after all numbers were run"))
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &BingoGame) -> Result<u32, ParseError> {
    let game = input.numbers.clone();
    let mut boards = input.boards.clone();
    let mut done_boards = vec![];

    for n in &game {
        for i in 0..boards.len() {
            boards[i].mark(*n);

            if boards[i].done() && !done_boards.contains(&i) {
                done_boards.push(i);
            }

            if done_boards.len() == boards.len() {
                if let Some(last) = done_boards.last() {
                    let unmarked = boards[*last].unmarked();
                    return Ok(unmarked * n);
                }
            }
        }
    }

    Err(ParseError::new("Not all boards are completed after all numbers were run"))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
    }

    fn input() -> Result<BingoGame, ParseError> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let bingo = input()?;
        Ok(assert_eq!(4512, solve_part1(&bingo)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let bingo = input()?;
        Ok(assert_eq!(1924, solve_part2(&bingo)?))
    }
}
