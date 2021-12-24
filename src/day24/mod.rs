use crate::utils::ParseError;
use memoize::memoize;

#[aoc_generator(day24)]
pub fn input_generator(_input: &str) -> Result<i64, ParseError> {
    Ok(0)
}

const CONF: [[i64; 3]; 14] = [
    // #01 AAA = 1, BBB = 11, CCC = 8
    [1, 11, 8],
    // #02 AAA = 1, BBB = 14, CCC = 13
    [1, 14, 13],
    // #03 AAA = 1, BBB = 10, CCC = 2
    [1, 10, 2],
    // #04 AAA = 26, BBB = 0, CCC = 7
    [26, 0, 7],
    // #05 AAA = 1, BBB = 12, CCC = 11
    [1, 12, 11],
    // #06 AAA = 1, BBB = 12, CCC = 4
    [1, 12, 4],
    // #07 AAA = 1, BBB = 12, CCC = 13
    [1, 12, 13],
    // #08 AAA = 26, BBB = -8, CCC = 13
    [26, -8, 13],
    // #09 AAA = 26, BBB = -9, CCC = 10
    [26, -9, 10],
    // #10 AAA = 1, BBB = 11, CCC = 1
    [1, 11, 1],
    // #11 AAA = 26, BBB = 0, CCC = 2
    [26, 0, 2],
    // #12 AAA = 26, BBB = -5, CCC = 14
    [26, -5, 14],
    // #13 AAA = 26, BBB = -6, CCC = 6
    [26, -6, 6],
    // #14 AAA = 26, BBB = -12, CCC = 14
    [26, -12, 14],
];


fn cell(w: i64, z: i64, i: usize) -> i64 {
    let x = CONF[i][1] + z % 26;
    let mut z = z / CONF[i][0];

    if x != w {
        z *= 26;
        z += w + CONF[i][2];
    }

    z
}

#[memoize]
fn monad(z: i64, i: usize) -> Vec<Vec<i64>> {
    if i == 14 {
        if z == 0 {
            return vec![vec![]];
        } else {
            return vec![];
        }
    }

    let next = z % 26 + CONF[i][1];
    let next_digits = if 0 < next && next < 10 {
        vec![next]
    } else {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    };

    let mut model_nums = vec![];
    for d in next_digits.into_iter() {
        let z_next = cell(d, z, i);
        let mut models = monad(z_next, i + 1);
        for model in &mut models {
            model.push(d);
        }
        model_nums.append(&mut models);
    }

    model_nums
}

fn to_num(v: Vec<i64>) -> i64 {
    let l = v.len();

    let mut result = 0;
    for (i, d) in v.iter().enumerate() {
        let p = (l - 1) - i;
        result += d * (10 as i64).pow(p as u32);
    }
    result
}

// tried: 51131616112781
// tried: 59998494939729
#[aoc(day24, part1)]
pub fn solve_part1(_input: &i64) -> Result<i64, ParseError> {
    let numbers = monad(0, 0);

    let max = numbers.into_iter().map(|v| to_num(v.into_iter().rev().collect::<Vec<_>>())).max().unwrap();

    Ok(max)
}

#[aoc(day24, part2)]
pub fn solve_part2(_input: &i64) -> Result<i64, ParseError> {
    let numbers = monad(0, 0);

    let min = numbers.into_iter().map(|v| to_num(v.into_iter().rev().collect::<Vec<_>>())).min().unwrap();

    Ok(min)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<i64, ParseError> {
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
