use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .map(|line| line.parse::<u32>().expect("non-integer in input!"))
        .collect::<Vec<u32>>()
}

#[aoc(day1, part1)]
pub fn part1(list: &Vec<u32>) -> Result<u32, &'static str> {
    // indices i, j
    // values a, b -> a + b = 2020
    let mut i = 0;
    let mut j;
    while i < list.len() {
        let a = list[i];
        let b = 2020 - a;
        j = i + 1;
        while j < list.len() {
            if list[j] == b {
                return Ok(a * b)
            }
            j += 1;
        }
        i += 1;
    }
    Err("no pair adding to 2020")
}

#[aoc(day1, part2)]
pub fn part2(list: &Vec<u32>) -> Result<u32, &'static str> {
    // indices i, j, k
    // values a, b, c -> a + b + c = 2020
    let mut i = 0;
    let mut j;
    let mut k;
    while i < list.len() {
        let a = list[i];
        j = i + 1;
        while j < list.len() {
            let b = list[j];
            if a + b >= 2020 {
                j += 1;
                continue;
            }
            let c = (2020 - b) - a;
            k = j + 1;
            while k < list.len() {
                if list[k] == c {
                    return Ok(a * b * c)
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    Err("no pair adding to 2020")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(514579, part1(&vec![1721, 979, 366, 299, 675, 1456]).unwrap());
    }

    #[test]
    fn part2_works() {
        assert_eq!(241861950, part2(&vec![1721, 979, 366, 299, 675, 1456]).unwrap());
    }
}