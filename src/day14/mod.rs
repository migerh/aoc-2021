use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use crate::utils::ParseError;

#[derive(Debug)]
pub struct Rule {
    first: char,
    second: char,
    insert: char,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(?P<first>.)(?P<second>.) -> (?P<insert>.)$").unwrap();
        }

        let (first, second, insert) = RE.captures(s).and_then(|cap| {
            let first = cap.name("first").map(|v| v.as_str().chars().take(1).collect::<Vec<_>>())?[0];
            let second = cap.name("second").map(|v| v.as_str().chars().take(1).collect::<Vec<_>>())?[0];
            let insert = cap.name("insert").map(|v| v.as_str().chars().take(1).collect::<Vec<_>>())?[0];

            Some((first, second, insert))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self { first, second, insert })
    }
}

#[derive(Debug)]
pub struct Formula {
    template: Vec<char>,
    rules: Vec<Rule>,
}

impl Formula {
    pub fn find_insert(&self, first: char, second: char) -> Result<char, ParseError> {
        let rule = self.rules.iter().find(|v| v.first == first && v.second == second).ok_or(ParseError::new("Could not find rule"))?;

        Ok(rule.insert)
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Formula, ParseError> {
    let template = input
        .lines()
        .take(1)
        .map(|v| v.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let rules = input
        .lines()
        .skip(2)
        .filter(|s| *s != "")
        .map(|s| Rule::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(Formula { template, rules })
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Formula) -> Result<i32, ParseError> {
    let mut result = input.template.clone();

    for _ in 0..10 {
        let mut inserts = vec![];
        for w in result.windows(2) {
            let insert = input.find_insert(w[0], w[1])?;
            inserts.push(insert);
        }

        result.reserve(inserts.len()+2);
        for (i, v) in inserts.iter().enumerate() {
            result.insert(i * 2 + 1, *v);
        }
    }

    let mut hist = HashMap::new();
    for c in result {
        hist.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let max = hist.iter().map(|(_, v)| v).max().ok_or(ParseError::new("Cant find max"))?;
    let min = hist.iter().map(|(_, v)| v).min().ok_or(ParseError::new("Cant find min"))?;

    Ok(max-min)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Formula) -> Result<usize, ParseError> {
    let mut map = HashMap::new();
    let last = input.template.iter().last().ok_or(ParseError::new("Has no last char"))?;

    for w in input.template.windows(2) {
        map.entry((w[0], w[1])).and_modify(|v| *v += 1).or_insert(1);
    }

    for _ in 0..40 {
        for (k, v) in map.clone() {
            let insert = input.find_insert(k.0, k.1)?;
            map.entry(k).and_modify(|s| *s -= v);
            map.entry((k.0, insert)).and_modify(|s| *s += v).or_insert(v);
            map.entry((insert, k.1)).and_modify(|s| *s += v).or_insert(v);
        }
    }

    let mut hist = HashMap::new();
    for (k, v) in map {
        hist.entry(k.0).and_modify(|s| *s += v).or_insert(v);
    }
    hist.entry(*last).and_modify(|s| *s += 1).or_insert(1);

    let max = hist.iter().map(|(_, v)| v).max().ok_or(ParseError::new("Cant find max"))?;
    let min = hist.iter().map(|(_, v)| v).min().ok_or(ParseError::new("Cant find min"))?;

    Ok(max-min)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    }

    fn input() -> Result<Formula, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(1588, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(2188189693529, solve_part2(&data)?))
    }
}
