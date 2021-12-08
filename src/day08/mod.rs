use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::ParseError;

type Item = Signal;

#[derive(Debug)]
pub struct Signal {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

impl FromStr for Signal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stuff = s.split("|")
            .filter(|s| *s != "")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();

        let input = stuff[0].split(" ").filter(|s| *s != "").map(|v| v.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let output = stuff[1].split(" ").filter(|s| *s != "").map(|v| v.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        Ok(Signal { input, output })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Vec<Item>, ParseError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| Signal::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day8, part1)]
pub fn solve_part1(signals: &Vec<Item>) -> Result<usize, ParseError> {
    let output = signals.iter().map(|s| s.output.clone()).collect::<Vec<_>>();
    let relevant_lengths: Vec<usize> = vec![2, 4, 3, 7];

    Ok(output
        .iter()
        .map(|s| s
            .iter()
            .filter(|t| relevant_lengths.contains(&t.len()))
            .count())
        .sum())
}

pub fn how_often(c: char, input: &Vec<Vec<char>>) -> usize {
    input.iter().filter(|v| v.contains(&c)).count()
}

pub fn decode_digit(map: &HashMap<char, char>, code: &Vec<char>) -> Result<usize, ParseError> {
    let mut codeout = code
        .iter()
        .map(|c| map.get(&c).ok_or(ParseError::new("Cannot map code")))
        .collect::<Result<Vec<_>, ParseError>>()?
        .into_iter()
        .map(|v| *v)
        .collect::<Vec<_>>();

    codeout.sort();
    let code_sorted = codeout.iter().collect::<String>();

    Ok(match code_sorted.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => Err(ParseError::new("Unknown wire configuration"))?,
    })
}

pub fn decode(map: &HashMap<char, char>, code: &Vec<Vec<char>>) -> Result<usize, ParseError> {
    let len = code.len();

    code.iter().enumerate()
        .map(|(i, s)| {
            let d = decode_digit(map, &s)?;
            Ok((10 as usize).pow((len - 1 - i) as u32) * d)
        })
        .try_fold(0, |acc, d: Result<usize, ParseError>| Ok(acc + d?))
}

#[aoc(day8, part2)]
pub fn solve_part2(signals: &Vec<Item>) -> Result<usize, ParseError> {
    let input = signals.iter().map(|s| s.input.clone()).collect::<Vec<_>>();
    let output = signals.iter().map(|s| s.output.clone()).collect::<Vec<_>>();
    let error = ParseError::new("Could not find wiring configuration");

    let mut sum = 0;
    for (k, i) in input.iter().enumerate() {
        let mut map: HashMap<char, char> = HashMap::new();

        // 1 has 2 wires, identify c & f
        // across all digits, c appears 8 times, f appears 9 times
        let one = i.iter().filter(|v| v.len() == 2).next().ok_or(error.clone())?;
        if how_often(one[0], i) == 8 {
            map.entry(one[0]).or_insert('c');
            map.entry(one[1]).or_insert('f');
        } else {
            map.entry(one[1]).or_insert('c');
            map.entry(one[0]).or_insert('f');
        }

        // 7 has 3 wires, identify a
        // it's the only wire in 7 not yet identified
        let seven = i.iter().filter(|v| v.len() == 3).next().ok_or(error.clone())?;
        let unmapped = seven.iter().filter(|v| !map.contains_key(v)).next().ok_or(error.clone())?;
        map.entry(*unmapped).or_insert('a');

        // 4 has 2 unidentified wires, identify b and d
        // across all digits, b appears 6 times, d appears 7 times
        let four = i.iter().filter(|v| v.len() == 4).next().ok_or(error.clone())?;
        let unmapped = four.iter().filter(|v| !map.contains_key(v)).cloned().collect::<Vec<_>>();
        if how_often(unmapped[0], i) == 6 {
            map.entry(unmapped[0]).or_insert('b');
            map.entry(unmapped[1]).or_insert('d');
        } else {
            map.entry(unmapped[1]).or_insert('b');
            map.entry(unmapped[0]).or_insert('d');
        }

        // pick 8, identify e and g
        // across all digits, e appears 4 times, g appears 7 times
        let eight = i.iter().filter(|v| v.len() == 7).next().ok_or(error.clone())?;
        let unmapped = eight.iter().filter(|v| !map.contains_key(v)).cloned().collect::<Vec<_>>();
        if how_often(unmapped[0], i) == 4 {
            map.entry(unmapped[0]).or_insert('e');
            map.entry(unmapped[1]).or_insert('g');
        } else {
            map.entry(unmapped[1]).or_insert('e');
            map.entry(unmapped[0]).or_insert('g');
        }

        let out = output[k].iter().map(|v| v.clone()).collect::<Vec<_>>();
        sum += decode(&map, &out)?;
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    fn input() -> Result<Vec<Item>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(26, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(61229, solve_part2(&data)?))
    }
}
