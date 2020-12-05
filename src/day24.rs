use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<u32> {
    unimplemented!("")
    // input.lines()
    // ...
}

#[aoc(day24, part1)]
pub fn part1(list: &Vec<usize>) -> Result<usize, &'static str> {
    unimplemented!("")
}

#[aoc(day24, part2)]
pub fn part2(list: &Vec<usize>) -> Result<usize, &'static str> {
    unimplemented!("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn part1_works() {
        assert_eq!(0, part1(&vec![1]).unwrap());
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(0, part2(&vec![2]).unwrap());
    }
}
