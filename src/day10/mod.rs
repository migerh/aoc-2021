use std::collections::HashMap;
use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>, ParseIntError> {
    Ok(input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

enum ParseResult {
    Corrupted(char),
    Incomplete(Vec<char>),
    Ok,
}

fn parse_lines(input: &Vec<Vec<char>>) -> Result<Vec<ParseResult>, ParseError> {
    let open = vec!['{', '[', '(', '<'];
    let close = vec!['}', ']', ')', '>'];

    let mut result = vec![];

    for line in input {
        let mut stack = vec![];
        let mut ok = true;
        for c in line {
            if open.contains(c) {
                stack.push(*c);
                continue;
            }

            let index = close.iter().position(|v| v == c).ok_or(ParseError::new("Illegal character"))?;
            let v = stack.pop().ok_or(ParseError::new("Stack is empty"))?;
            let o = open.get(index).ok_or(ParseError::new("Could not find corresponding opening char"))?;
            if *o != v {
                result.push(ParseResult::Corrupted(*c));
                ok = false;
                break;
            }
        }

        if ok && stack.is_empty() {
            result.push(ParseResult::Ok);
        } else if ok {
            result.push(ParseResult::Incomplete(stack));
        }
    }

    Ok(result)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> Result<usize, ParseError> {
    let score_map = vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect::<HashMap<_,_>>();
    let score = parse_lines(input)?.iter()
        .filter_map(|v| if let ParseResult::Corrupted(c) = v {
            Some(c)
        } else {
            None
        })
        .try_fold(0, |sum, v| -> Result<usize, ParseError> {
            Ok(sum + score_map.get(v).ok_or(ParseError::new("Unexpcted closing brace"))?)
        })?;

    Ok(score)
}

fn score_incomplete_line(stack: &Vec<char>) -> Result<usize, ParseError> {
    let score_map = vec![('(', 1), ('[', 2), ('{', 3), ('<', 4)].into_iter().collect::<HashMap<_,_>>();
    stack.iter()
        .rev()
        .try_fold(0, |sum, v| -> Result<usize, ParseError> {
            Ok(sum * 5 + score_map.get(v).ok_or(ParseError::new(format!("Cant score {}", v).as_str()))?)
        })
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> Result<usize, ParseError> {
    let parser_result = parse_lines(input)?;
    let mut scores = parser_result.iter()
        .filter_map(|v| if let ParseResult::Incomplete(stack) = v { Some(stack) } else { None })
        .map(|v| score_incomplete_line(v))
        .collect::<Result<Vec<_>, _>>()?;

    scores.sort();
    let median = scores[scores.len() / 2];

    Ok(median)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    fn input() -> Result<Vec<Vec<char>>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(26397, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(288957, solve_part2(&data)?))
    }
}
