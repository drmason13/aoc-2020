use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, convert::TryInto};

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .expect("Failed to parse input")
}

/// The positions that each number has appeared in, sorted from lowest (first) to highest (most recently)
type Positions = HashMap<u32, Vec<u32>>;

fn starting_numbers(list: &Vec<u32>) -> (Positions, u32) {
    assert!(list.len() > 0);
    let mut positions: Positions = HashMap::new();
    let mut last_inserted = 0; // we've guaranteed that this will be overwritten
    for (i, n) in list.iter().enumerate() {
        let x = positions.entry(*n).or_insert(Vec::new());
        last_inserted = *n;
        x.push((i + 1).try_into().unwrap());
    }
    (positions, last_inserted)
}

pub fn run(list: &Vec<u32>, limit: u32) -> u32 {
    let (mut positions, mut last_spoken) = starting_numbers(list);
    let mut count = list.len().try_into().unwrap();

    while count < limit {
        let last_seen_positions = positions.get(&last_spoken).unwrap();
        let len = last_seen_positions.len();
        last_spoken = if len < 2 {
            // new number, so we speak 0
            0
        } else {
            // repeat number, we speak the time since *the time it was previously spoken* (i.e. not immediately prior to now!)
            let previous_position = last_seen_positions[len - 2];
            count - previous_position
        };
        // record what we just spoke
        let y = positions.entry(last_spoken).or_insert(Vec::new());
        count += 1;
        y.push(count);
    }
    last_spoken
}

#[aoc(day15, part1)]
pub fn part1(list: &Vec<u32>) -> u32 {
    run(list, 2020)
}

#[aoc(day15, part2)]
pub fn part2(list: &Vec<u32>) -> u32 {
    run(list, 30000000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        // Given the starting numbers 1,3,2, the 2020th number spoken is 1.
        assert_eq!(1, part1(&vec![1, 3, 2]));
        // Given the starting numbers 2,1,3, the 2020th number spoken is 10.
        assert_eq!(10, part1(&vec![2, 1, 3]));
        // Given the starting numbers 1,2,3, the 2020th number spoken is 27.
        assert_eq!(27, part1(&vec![1, 2, 3]));
        // Given the starting numbers 2,3,1, the 2020th number spoken is 78.
        assert_eq!(78, part1(&vec![2, 3, 1]));
        // Given the starting numbers 3,2,1, the 2020th number spoken is 438.
        assert_eq!(438, part1(&vec![3, 2, 1]));
        // Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
        assert_eq!(1836, part1(&vec![3, 1, 2]));
    }

    #[test]
    #[ignore] // too slow ;)
    fn part2_works() {
        // Given the starting numbers 1,3,2, the 30000000th number spoken is 2578.
        assert_eq!(2578, part2(&vec![1, 3, 2]));
        // Given the starting numbers 2,1,3, the 30000000th number spoken is 3544142.
        assert_eq!(3544142, part2(&vec![2, 1, 3]));
        // Given the starting numbers 1,2,3, the 30000000th number spoken is 261214.
        assert_eq!(261214, part2(&vec![1, 2, 3]));
        // Given the starting numbers 2,3,1, the 30000000th number spoken is 6895259.
        assert_eq!(6895259, part2(&vec![2, 3, 1]));
        // Given the starting numbers 3,2,1, the 30000000th number spoken is 18.
        assert_eq!(18, part2(&vec![3, 2, 1]));
        // Given the starting numbers 3,1,2, the 30000000th number spoken is 362.
        assert_eq!(362, part2(&vec![3, 1, 2]));
    }
}
