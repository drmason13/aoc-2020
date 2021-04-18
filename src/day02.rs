use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

pub trait PasswordPolicy {
    fn validate(&self, password: &str) -> bool;
}

#[derive(PartialEq, Debug, Clone)]
pub struct SledRentalPasswordPolicy {
    min: usize,
    max: usize,
    character: char,
}

impl PasswordPolicy for SledRentalPasswordPolicy {
    fn validate(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.character).count();
        count >= self.min && count <= self.max
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct TobogganCorporatePasswordPolicy {
    indices: (usize, usize),
    character: char,
}

impl PasswordPolicy for TobogganCorporatePasswordPolicy {
    fn validate(&self, password: &str) -> bool {
        let char_list = password.chars().collect::<Vec<char>>();
        match (
            char_list[self.indices.0] == self.character,
            char_list[self.indices.1] == self.character,
        ) {
            (true, false) => true,
            (false, true) => true,
            (true, true) => false,
            (false, false) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordBundle<P: PasswordPolicy> {
    password: String,
    password_policy: P,
}

impl FromStr for SledRentalPasswordPolicy {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(&['-', ' '][..]);
        let min = iter
            .next()
            .ok_or("missing min")?
            .parse::<usize>()
            .map_err(|_| "min part was not an integer")?;
        let max = iter
            .next()
            .ok_or("missing max")?
            .parse::<usize>()
            .map_err(|_| "max part was not an integer")?;
        let character = iter
            .next()
            .ok_or("missing character part")?
            .chars()
            .next()
            .ok_or("missing character")?;

        Ok(SledRentalPasswordPolicy {
            min,
            max,
            character,
        })
    }
}

impl FromStr for TobogganCorporatePasswordPolicy {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(&['-', ' '][..]);
        let i = iter
            .next()
            .ok_or("missing first index")?
            .parse::<usize>()
            .map_err(|_| "first index was not an integer")?;
        let j = iter
            .next()
            .ok_or("missing second index")?
            .parse::<usize>()
            .map_err(|_| "second index was not an integer")?;
        let character = iter
            .next()
            .ok_or("missing character part")?
            .chars()
            .next()
            .ok_or("missing character")?;

        Ok(TobogganCorporatePasswordPolicy {
            indices: (i - 1, j - 1),
            character,
        })
    }
}

impl FromStr for PasswordBundle<SledRentalPasswordPolicy> {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(':');
        let password_policy = iter.next().ok_or("missing password_policy")?.parse()?;
        let password = iter
            .next()
            .ok_or("missing password")?
            .chars()
            .skip(1)
            .collect::<String>();

        Ok(PasswordBundle {
            password,
            password_policy,
        })
    }
}
impl FromStr for PasswordBundle<TobogganCorporatePasswordPolicy> {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(':');
        let password_policy = iter.next().ok_or("missing password_policy")?.parse()?;
        let password = iter
            .next()
            .ok_or("missing password")?
            .chars()
            .skip(1)
            .collect::<String>();

        Ok(PasswordBundle {
            password,
            password_policy,
        })
    }
}

impl<P: PasswordPolicy> PasswordBundle<P> {
    fn validate(&self) -> bool {
        self.password_policy.validate(&self.password)
    }
}

#[aoc_generator(day2, part1)]
pub fn input_generator_part1(input: &str) -> Vec<PasswordBundle<SledRentalPasswordPolicy>> {
    input
        .lines()
        .map(|line| line.parse().expect("failed to parse input"))
        .collect()
}

#[aoc_generator(day2, part2)]
pub fn input_generator_part2(input: &str) -> Vec<PasswordBundle<TobogganCorporatePasswordPolicy>> {
    input
        .lines()
        .map(|line| line.parse().expect("failed to parse input"))
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(password_bundles: &Vec<PasswordBundle<SledRentalPasswordPolicy>>) -> usize {
    password_bundles.iter().filter(|p| p.validate()).count()
}

#[aoc(day2, part2)]
pub fn part2(password_bundles: &Vec<PasswordBundle<TobogganCorporatePasswordPolicy>>) -> usize {
    password_bundles.iter().filter(|p| p.validate()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sled_rental_password_policy() {
        let input = "1-3 c: abcde";
        let password_bundle = input
            .parse::<PasswordBundle<SledRentalPasswordPolicy>>()
            .expect("failed to parse input");
        assert_eq!(password_bundle.password, String::from("abcde"));
        assert_eq!(
            password_bundle.password_policy,
            SledRentalPasswordPolicy {
                min: 1,
                max: 3,
                character: 'c'
            }
        );
    }

    #[test]
    fn parse_toboggan_corporate_password_policy() {
        let input = "1-3 c: abcde";
        let password_bundle = input
            .parse::<PasswordBundle<TobogganCorporatePasswordPolicy>>()
            .expect("failed to parse input");
        assert_eq!(password_bundle.password, String::from("abcde"));
        assert_eq!(
            password_bundle.password_policy,
            TobogganCorporatePasswordPolicy {
                indices: (0, 2),
                character: 'c'
            }
        );
    }

    #[test]
    fn part1_works() {
        let input = "1-3 a: abcde\n\
            1-3 b: cdefg\n\
            2-9 c: ccccccccc";

        let password_bundles = input_generator_part1(input);
        assert_eq!(part1(&password_bundles), 2);
    }

    #[test]
    fn part2_works() {
        let input = "1-3 a: abcde\n\
            1-3 b: cdefg\n\
            2-9 c: ccccccccc";

        let password_bundles = input_generator_part2(input);
        assert_eq!(part2(&password_bundles), 1);
    }
}
