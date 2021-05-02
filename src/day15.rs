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

/// The position that each number last appeared
type Positions = HashMap<u32, u32>;

struct SpokenRecord {
    prev: u32,
    prev_position: u32,
    record: Positions,
}

impl SpokenRecord {
    fn new(list: &Vec<u32>) -> Self {
        assert!(list.len() > 0);
        let mut record: Positions = HashMap::new();

        let mut prev: Option<u32> = None;
        let mut prev_position: Option<u32> = None;

        for (next_position, next) in list.iter().enumerate() {
            if let Some(value) = prev {
                let x = record.entry(value).or_default();
                *x = (prev_position.unwrap()).try_into().unwrap();
            }

            prev = Some(*next);
            prev_position = Some(next_position.try_into().unwrap());
        }

        SpokenRecord {
            prev: prev.unwrap(),
            prev_position: prev_position.unwrap().try_into().unwrap(),
            record,
        }
    }
}

impl Iterator for SpokenRecord {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // work out what to speak next based on what we spoke previously
        let speak_next = match self.record.get(&self.prev) {
            None => 0,
            Some(position_in_record) => self.prev_position - position_in_record,
        };

        // update the record with what we spoke previously
        let x = self.record.entry(self.prev).or_default();
        *x = self.prev_position;

        // update what we spoke previously to what we have now just spoken
        self.prev = speak_next;
        self.prev_position += 1;

        // return what we hath speaketh
        Some(speak_next)
    }
}

pub fn run(list: &Vec<u32>, limit: u32) -> u32 {
    let spoken_record = SpokenRecord::new(list);
    spoken_record
        .skip((limit as usize - 1) - list.len())
        .next()
        .unwrap()
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
        // Given the starting numbers 0,3,6, the 2020th number spoken is 436.
        assert_eq!(436, part1(&vec![0, 3, 6]));
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
    #[ignore]
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
