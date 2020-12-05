use aoc_runner_derive::{aoc_generator, aoc};

use std::str::FromStr;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Passport(HashMap<PassportField, String>);

impl Passport {
    fn validate(&self) -> bool {
        self.0.get(&PassportField::Byr).is_some() &&
        self.0.get(&PassportField::Iyr).is_some() &&
        self.0.get(&PassportField::Eyr).is_some() &&
        self.0.get(&PassportField::Hgt).is_some() &&
        self.0.get(&PassportField::Hcl).is_some() &&
        self.0.get(&PassportField::Ecl).is_some() &&
        self.0.get(&PassportField::Pid).is_some()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum PassportField {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl FromStr for PassportField {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "byr" => Ok(PassportField::Byr),
            "iyr" => Ok(PassportField::Iyr),
            "eyr" => Ok(PassportField::Eyr),
            "hgt" => Ok(PassportField::Hgt),
            "hcl" => Ok(PassportField::Hcl),
            "ecl" => Ok(PassportField::Ecl),
            "pid" => Ok(PassportField::Pid),
            "cid" => Ok(PassportField::Cid),
            _ => Err("Invalid Passport field")
        }
    }
}

impl PassportField {
    fn validate(&self, value: &str) -> bool {
        println!("field: {:?}, value: {}", &self, value);
        match self {            
            PassportField::Byr => if let Ok(year) = value.parse::<u32>() {
                year >= 1920 && year <= 2002
            } else {
                false
            },
            PassportField::Iyr => if let Ok(year) = value.parse::<u32>() {
                year >= 2010 && year <= 2020
            } else {
                false
            }, 
            PassportField::Eyr => if let Ok(year) = value.parse::<u32>() {
                year >= 2020 && year <= 2030
            } else {
                false
            },
            PassportField::Hgt => {
                let (height, unit) = value.split_at(value.len() - 2);
                if let Ok(height) = height.parse::<u32>() {
                    match unit {
                        "cm" => { height >= 150 && height <= 193 },
                        "in" => { height >= 59 && height <= 76 },
                        _ => { eprintln!("invalid unit for height field"); false },
                    }
                } else {
                    eprintln!("invalid integer for height field");
                    false
                }
            },
            PassportField::Hcl => {
                let mut chars = value.chars();
                if let Some('#') = chars.next() {
                    chars.take_while(|c| match c {
                        '0'..='9' | 'a'..='f' => true,
                        _ => false
                    }).count() == 6
                } else {
                    eprintln!("Hair Color field invalid because it did not start with a '#'");
                    false
                }
            }
            PassportField::Ecl => {
                match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            }
            PassportField::Pid => value.chars().filter(|c| c.is_numeric()).count() == 9,
            PassportField::Cid => true,
        }
    }
}

impl FromStr for Passport {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut passport = HashMap::new();
        let mut kv_iter = input.split_whitespace();
        while let Some(kv) = kv_iter.next() {
            let mut kv = kv.split(':');
            match kv.next() {
                Some(field) => {
                    if let Ok(field) = field.parse::<PassportField>() {
                        match kv.next() {
                            Some(value) => {
                                passport.insert(field, value.to_owned());
                            }
                            None => { return Err("Missing value") },
                        }
                    } else {
                        return Err("Unknown passport field")
                    }
                },
                None => { return Err("Missing Passport field") },
            }
        }

        Ok(Passport(passport))
    }
}

#[aoc_generator(day4)]
pub fn input_generator<'a>(input: &'a str) -> Vec<Result<Passport, &'static str>> {
    input.split("\n\n").map(|passport| {
        passport.parse::<Passport>()
    }).collect::<Vec<_>>()
}

#[aoc(day4, part1)]
pub fn part1(passports: &Vec<Result<Passport, &'static str>>) -> usize {
    passports.iter().filter_map(|p| p.as_ref().ok()).filter(|&p| p.validate()).count()
}

#[aoc(day4, part2)]
pub fn part2(passports: &Vec<Result<Passport, &'static str>>) -> usize {
    passports.iter().filter_map(|p| p.as_ref().ok()).filter(|&p| p.validate()).filter(|p| {
        p.0.iter().filter(|(field, value)| {
            !field.validate(value)
        }).count() == 0
    }).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_PART1: &'static str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

const TEST_INPUT_PART2_INVALID: &'static str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

const TEST_INPUT_PART2_VALID: &'static str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn part1_works() {
        assert_eq!(2, part1(&input_generator(TEST_INPUT_PART1)));
    }

    #[test]
    fn part2_works() {
        assert_eq!(0, part2(&input_generator(TEST_INPUT_PART2_INVALID)));
        assert_eq!(4, part2(&input_generator(TEST_INPUT_PART2_VALID)));
    }
}
