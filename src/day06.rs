use aoc_runner_derive::aoc;

use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut unique_answers = HashSet::new();
            for person in group.lines() {
                for answer in person.chars() {
                    unique_answers.insert(answer);
                }
            }
            unique_answers
        })
        .map(|x| x.len())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut group_iter = group.lines().map(|person| {
                // each person is a set of answers
                person.chars().collect::<HashSet<char>>()
            });
            // we want to return the intersecting answers of all the people in each group
            let intersecting_answers = group_iter.next().unwrap();
            group_iter.fold(intersecting_answers, |acc, set| {
                acc.intersection(&set).cloned().collect()
            })
        })
        .map(|x| x.len())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_works() {
        assert_eq!(11, part1(&TEST_INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!(6, part2(&TEST_INPUT));
    }
}
