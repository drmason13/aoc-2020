use aoc_runner_derive::{aoc_generator, aoc};
use parse_display::FromStr;

use std::collections::HashSet;

/*
#[derive(FromStr, PartialEq, Debug)]
#[display("acc {amount}")]
struct Acc {
    amount: isize,
}

#[derive(FromStr, PartialEq, Debug)]
#[display("jmp +{offset}")]
struct Jump {
    offset: isize,
}
*/

#[derive(Clone, FromStr, PartialEq, Debug)]
pub enum Instruction {
    #[from_str(regex = r"acc \+?(?P<0>-?[0-9]+)")]
    Acc(i32),
    #[from_str(regex = r"jmp \+?(?P<0>-?[0-9]+)")]
    Jump(i32),
    #[from_str(regex = r"nop \+?(?P<0>-?[0-9]+)")]
    Noop(i32),
}

/// visited records every unique instruction_pointer (i.e. Program index) that has already been executed
/// in order to detect infinite loops at runtime
#[derive(Default, Debug, PartialEq)]
pub struct Machine {
    instruction_pointer: usize,
    accumulator: i32,
    visited: HashSet<usize>
}

/// Program stores instructions and state
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
    state: ProgramState
}

/// Records the most recent shuffle made to the Program
/// for the purposes of brute forcing Program alterations
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramState {
    // the index of the next instruction to change
    next: usize,
    // the index of the most recently changed instruction (if any)
    prev: Option<usize>,
    repeat: bool,
}

impl Program {
    fn get(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    // swaps Jumps for Noops, and maintains only 1 swap at a time
    // self.state.repeat is true if we are running shuffle recursively to skip Instructions that shouldn't be shuffled
    // i.e. Acc
    // it is set by the shuffle function where necessary
    fn shuffle(&mut self) {
        // swap back previous
        if self.state.repeat == false {
            if let Some(prev) = self.state.prev {
                match self.instructions.get(prev) {
                    None => unreachable!(),
                    Some(Instruction::Jump(x)) => self.instructions[prev] = Instruction::Noop(*x),
                    Some(Instruction::Noop(x)) => self.instructions[prev] = Instruction::Jump(*x),
                    Some(Instruction::Acc(_)) => unreachable!(),
                }
            }
        }
        // swap current
        match self.instructions.get(self.state.next) {
            None => panic!("we tried all of the shuffles and now we don't know what to do!"),
            Some(Instruction::Jump(x)) => {
                self.state.prev = Some(self.state.next);
                self.instructions[self.state.next] = Instruction::Noop(*x);
            },
            Some(Instruction::Noop(x)) => {
                self.state.prev = Some(self.state.next);
                self.instructions[self.state.next] = Instruction::Jump(*x);
            },
            Some(Instruction::Acc(_)) => {self.state.repeat = true; self.state.next += 1; self.shuffle()},
        }
        self.state.next += 1;
        self.state.repeat = false;
    }
}

impl Machine {
    fn new() -> Machine {
        Machine::default()
    }

    /// Step through one instruction of the program at the instruction_pointer
    /// returns Some(x) if the instruction_pointer was within the bounds of the Program
    /// where x is the value of the accumulator after running that instruction
    /// returns None otherwise indicating that the program is finished.
    fn step(&mut self, program: &Program) -> Option<i32> {
        if let Some(next) = program.get(self.instruction_pointer) {
            if self.visited.insert(self.instruction_pointer) {
                &self.execute(next);
                Some(self.accumulator)
            } else {
                // instruction has been run before...
                // infinite loop detected - stop
                None
            }
        } else {
            None
        }
    }

    /// resets the machine and runs through the program until it finishes or finds an infinite loop
    /// returns Ok(accumulator) if finishes, returns Err(accumulator) if it detects an infinite loop.
    fn run(&mut self, program: &Program) -> Result<i32, i32> {
        self.instruction_pointer = 0;
        self.accumulator = 0;
        self.visited = HashSet::new();
        while let Some(next) = program.get(self.instruction_pointer) {
            if self.visited.insert(self.instruction_pointer) {
                &self.execute(next);
            } else {
                // instruction has been run before...
                // infinite loop detected - stop
                return Err(self.accumulator)
            }
        }
        Ok(self.accumulator)
    }

    /// Mutates self in order to do the instruction
    /// panics if the instruction pointer is set to a negative value by too large a negative jump
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Acc(x) => { self.accumulator += x; self.instruction_pointer += 1; },
            Instruction::Jump(x) => { self.instruction_pointer = (self.instruction_pointer as i32 + x) as usize; },
            Instruction::Noop(_) => { self.instruction_pointer += 1; },
        }
    }
}


/// This will output a Program
#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Program {
    Program {
        instructions: input.lines().map(|line| {
            line.parse::<Instruction>()
        }).collect::<Result<_,_>>().unwrap(),
        state: ProgramState {
            next: 0,
            prev: None,
            repeat: false,
        },
    }
}

#[aoc(day8, part1)]
pub fn part1(program: &Program) -> Option<i32> {
    let mut machine = Machine::new();

    let mut answer = None;

    while let Some(acc) = &machine.step(program) {
        println!("{}", acc);
        answer = Some(*acc)
    }
    println!("finished");
    answer
}

#[aoc(day8, part2)]
pub fn part2(program: &Program) -> Option<i32> {
    let mut machine = Machine::new();
    let mut program = (*program).clone();

    let mut answer = Err(0);

    while answer.is_err() {
        program.shuffle();
        answer = machine.run(&program);
    }
    answer.ok()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PROGRAM: &'static str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn program_parsing_works() {
        assert_eq!(vec![
            Instruction::Noop(0),
            Instruction::Acc(1),
            Instruction::Jump(4),
            Instruction::Acc(3),
            Instruction::Jump(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jump(-4),
            Instruction::Acc(6),
        ], input_generator(TEST_PROGRAM).instructions);
    }

    #[test]
    fn part1_works() {
        assert_eq!(5, part1(&input_generator(TEST_PROGRAM)).unwrap());
    }

    #[test]
    fn part2_works() {
        assert_eq!(8, part2(&input_generator(TEST_PROGRAM)).unwrap());
    }
}
