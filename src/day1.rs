#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> u32 {
    0
}