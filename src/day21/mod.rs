use std::cmp::max;
use std::collections::HashMap;
use crate::utils::ParseError;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Game {
    players: [usize; 2],
    scores: [usize; 2],
    player: usize,
}

impl Game {
    fn new(player1: usize, player2: usize) -> Self {
        let players = [player1, player2];
        let scores = [0, 0];
        let player = 0;
        Game { players, scores, player }
    }

    fn over(&self) -> bool {
        if self.scores[0] >= 1000 || self.scores[1] >= 1000 {
            true
        } else {
            false
        }
    }

    fn tick(&mut self, player: usize, sum: usize) {
        self.players[player] = (self.players[player] - 1 + sum) % 10 + 1;
        self.scores[player] += self.players[player];
    }
}

#[derive(Debug)]
pub struct Die {
    rolls: Vec<usize>,
}

impl Die {
    fn new() -> Self {
        let rolls = vec![];
        Die { rolls }
    }

    fn roll_det(&mut self) -> usize {
        let len = self.rolls.len();
        self.rolls.push(len % 100 + 1);
        len % 100 + 1
    }
}

#[aoc_generator(day21)]
pub fn input_generator(_input: &str) -> Game {
    // Game::new(4, 8)
    Game::new(1, 10)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Game) -> Result<usize, ParseError> {
    let mut game = input.clone();
    let mut player = 0;
    let mut die = Die::new();

    while !game.over() {
        let roll = die.roll_det() + die.roll_det() + die.roll_det();
        game.tick(player, roll);
        player = (player + 1) % 2;
    }

    let score = if game.scores[0] < 1000 {
        game.scores[0] * die.rolls.len()
    } else {
        game.scores[1] * die.rolls.len()
    };
    Ok(score)
}

fn number_of_wins(player: usize, games: &HashMap<Game, usize>) -> usize {
    games.iter().filter(|(game, _)| game.player == player).map(|(_, num)| num).sum()
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Game) -> Result<usize, ParseError> {
    let mut ongoing_games = HashMap::<Game, usize>::new();
    let initial_game = input.clone();
    ongoing_games.entry(initial_game).or_insert(1);

    let mut finished_games = HashMap::new();
    while ongoing_games.len() > 0 {
        let mut next_games = HashMap::new();

        for (game, num) in ongoing_games {
            for d1 in 1..=3 {
                for d2 in 1..=3 {
                    for d3 in 1..=3 {
                        let sum = d1 + d2 + d3;
                        let mut new_game = game.clone();
                        new_game.tick(game.player, sum);

                        if new_game.scores[game.player] >= 21 {
                            finished_games.entry(new_game).and_modify(|v| *v += num).or_insert(num);
                        } else {
                            new_game.player = (game.player + 1) % 2;
                            next_games.entry(new_game).and_modify(|v| *v += num).or_insert(num);
                        }
                    }
                }
            }
        }

        ongoing_games = next_games.clone();
    }

    let wins1 = number_of_wins(0, &finished_games);
    let wins2 = number_of_wins(1, &finished_games);

    Ok(max(wins1, wins2))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn input() -> Game {
        Game::new(4, 8)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input();
        Ok(assert_eq!(739785, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input();
        Ok(assert_eq!(444356092776315, solve_part2(&data)?))
    }
}
