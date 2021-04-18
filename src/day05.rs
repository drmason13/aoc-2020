use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

#[derive(Debug)]
pub struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let row_partition_len = 7;

        let mut row = 0;
        let mut column = 0;
        let mut error = None;
        let (row_partition, col_partition) = input.split_at(row_partition_len);

        row_partition
            .chars()
            .enumerate()
            .for_each(|(i, c)| match c {
                'F' => {} // Front / Lower half
                'B' => {
                    row += 2_usize.pow(6 - i as u32);
                } // Back / Upper Half
                _ => {
                    error = Some("Invalid character in row partition");
                }
            });

        if let Some(error) = error {
            return Err(error);
        }

        col_partition
            .chars()
            .enumerate()
            .for_each(|(i, c)| match c {
                'L' => {} // Left / Lower Half
                'R' => {
                    column += 2_usize.pow(2 - i as u32);
                } // Back / Upper Half
                _ => {
                    error = Some("Invalid character in row partition");
                }
            });

        if let Some(error) = error {
            return Err(error);
        }

        Ok(Seat { row, column })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    // WARNING: any invalid "Seat" input is simply ignored
    input
        .lines()
        .filter_map(|line| line.parse::<Seat>().ok())
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(list: &Vec<Seat>) -> usize {
    list.iter().map(|seat| seat.id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(list: &Vec<Seat>) -> usize {
    let mut seat_ids = list.iter().map(|seat| seat.id()).collect::<Vec<_>>();
    seat_ids.sort_unstable();

    seat_ids
        .as_slice()
        .windows(3)
        .filter_map(|w| {
            let a = w[0];
            let b = w[1];
            let c = w[2];
            if b != a + 1 {
                Some(a + 1)
            } else if c != b + 1 {
                Some(b + 1)
            } else {
                None
            }
        })
        .next()
        .expect("the missing id should be ours")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_SEATS: &'static str = "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn seat_parsing_works() {
        let expected = &[(70, 7, 567), (14, 7, 119), (102, 4, 820)];

        for (i, seat) in input_generator(EXAMPLE_SEATS).iter().enumerate() {
            assert_eq!(expected[i].0, seat.row);
            assert_eq!(expected[i].1, seat.column);
            assert_eq!(expected[i].2, seat.id());
        }
    }

    #[test]
    fn part1_works() {
        assert_eq!(820, part1(&input_generator(EXAMPLE_SEATS)));
    }
}
