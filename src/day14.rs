use aoc_runner_derive::{aoc_generator, aoc};
use std::str::FromStr;

use std::collections::HashMap;

// shamelessly taken from the once_cell documentation
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub struct Mask(Vec<MaskAction>);

pub enum MaskAction {
    SetTo0,
    SetTo1,
    Noop,
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        let binary = format!("{:036b}", value);
        let masked_binary = binary.chars().zip(self.0.iter()).map(|(v, m)| {
            match (v, m) {
                (_, MaskAction::SetTo0) => '0',
                (_, MaskAction::SetTo1) => '1',
                (v, MaskAction::Noop) => v,
            }
        }).collect::<String>();
        u64::from_str_radix(&masked_binary, 2).expect("unable to parse maksed binary as integer")
    }
}

#[derive(Debug)]
pub struct MaskParseError(String);

impl std::error::Error for MaskParseError {}

impl std::fmt::Display for MaskParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Mask {
    type Err = MaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask_regex = regex!("^mask = ([01X]+)$");
        let caps = mask_regex.captures(s).unwrap();
        let mask_actions = caps[1].chars().map(|c| match c {
            '0' => Ok(MaskAction::SetTo0),
            '1' => Ok(MaskAction::SetTo1),
            'X' => Ok(MaskAction::Noop),
            _ => Err(MaskParseError("Invalid bitmask representation".into()))
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Mask(mask_actions))
    }
}

pub struct Instruction {
    target: u64,
    value: u64,
}

#[derive(Debug)]
pub struct InstructionParseError(String);

impl std::error::Error for InstructionParseError {}

impl std::fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// let instructions = lines.map(|l| {
impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_regex = regex!(r#"^mem\[(\d+)\] = (\d+)$"#);
        let caps = instruction_regex.captures(s).ok_or(InstructionParseError(format!("invalid instruction: {}", s)))?;
        let target = caps.get(1).unwrap().as_str().parse::<u64>().map_err(|_| InstructionParseError(format!("invalid target in instruction: {}", s)))?;
        let value = caps.get(2).unwrap().as_str().parse::<u64>().map_err(|_| InstructionParseError(format!("invalid value in instruction: {}", s)))?;
        Ok(Instruction { target, value })
    }
}

#[derive(Debug)]
pub enum ParsingError {
    InstructionParseError(InstructionParseError),
    MaskParseError(MaskParseError),
}

impl std::error::Error for ParsingError {}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::InstructionParseError(x) => x.fmt(f),
            ParsingError::MaskParseError(x) => x.fmt(f),
        }
    }
}

pub enum InstructionOrMask {
    Instruction(Instruction),
    Mask(Mask),
}

fn parse_instruction_or_mask(s: &str) -> Result<InstructionOrMask, ParsingError> {
    s.parse::<Instruction>()
        .map(|x| InstructionOrMask::Instruction(x))
        .map_err(|e| ParsingError::InstructionParseError(e))
        .or_else(|_| {
            s.parse::<Mask>()
                .map_err(|e| ParsingError::MaskParseError(e))
                .map(|x| InstructionOrMask::Mask(x))
            }
        )
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<InstructionOrMask> {
    input.lines()
        .map(|l| parse_instruction_or_mask(l))
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse input")
}

#[aoc(day14, part1)]
pub fn part1(input: &Vec<InstructionOrMask>) -> Result<u64, &'static str> {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = None;
    for i in input {
        match i {
            InstructionOrMask::Mask(mask) => current_mask = Some(mask),
            InstructionOrMask::Instruction(instruction) => {
                let x = memory.entry(instruction.target).or_insert(0);
                *x = current_mask.expect("mask not initialised!").apply(instruction.value);
            }
        }
    }

    Ok(memory.values().sum())
}

#[aoc(day14, part2)]
pub fn part2(_input: &Vec<InstructionOrMask>) -> Result<u64, &'static str> {
    unimplemented!("")
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const PROGRAM: &'static str = indoc! {"
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0"};

    #[test]
    fn part1_works() {
        assert_eq!(165, part1(&input_generator(PROGRAM)).unwrap());
    }

    #[test]
    #[ignore]
    fn part2_works() {
        unimplemented!()
    }
}
