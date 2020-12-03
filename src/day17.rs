use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .map(|line| line.parse::<u32>().expect("non-integer in input!"))
        .collect::<Vec<u32>>()
}

#[aoc(day17, part1)]
pub fn solve(_list: &Vec<u32>) -> Result<u32, &'static str> {
    // ...
    Ok(0)
}
