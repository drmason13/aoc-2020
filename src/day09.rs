use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq)]
pub struct Xmas {
    numbers: Vec<u64>,
    cursor: usize,
    preamble_length: usize,
    weakness: Option<u64>,
    encryption_weakness: Option<u64>,
}

impl Xmas {
    fn new(preamble_length: usize) -> Self {
        Xmas {
            numbers: Vec::new(),
            cursor: 0,
            preamble_length,
            weakness: None,
            encryption_weakness: None,
        }
    }

    /// look at the next number and return it if it is a weakness
    /// A weakness can only be detected if we have finished the preamble
    fn process(&mut self, number: u64) -> Option<u64> {
        self.numbers.push(number);

        if self.numbers.len() > self.preamble_length {
            match self.validate() {
                Err(weakness) => {
                    //println!("{} is a weakness", weakness);
                    self.weakness = Some(weakness);
                    self.cursor += 1;
                },
                Ok(_) => {
                //Ok((a, b)) => {
                    //println!("Ok: {} + {} = {}", a, b, self.numbers.last().expect("numbers must not be empty"));
                    self.cursor += 1;
                },
            };
        }

        self.weakness
    }

    fn validate(&self) -> Result<(u64, u64), u64> {
        let target = self.numbers.last().expect("numbers must not be empty");
        let last_index = self.numbers.len() - 1;

        for i in self.cursor..last_index {
            if let Some(need) = target.checked_sub(self.numbers[i]) {
                //println!("searching... target: {}, have: {}, need: {}", target, self.numbers[i], need);
                if need == self.numbers[i] {
                    // exit early because now there is no way to make the target without using the same number twice (which isn't allowed)
                    continue
                }
                for j in (i + 1)..last_index {
                    if self.numbers[j] == need {
                        return Ok((self.numbers[i], self.numbers[j]))
                    }
                }
            } else {
                //println!("skipping... target: {}, have: {}", target, self.numbers[i]);
            }
        }
        // found the weakness: it had no preceeding pair that added to it
        Err(*target)
    }

    /// Attempts to crack the code by using the stored weakness
    /// Before cracking, find the weakness by processing input one at a time in order
    /// Returns None if no weakness is stored or no further weakness can be found using the stored weakness
    /// TODO: simple state machine to prevent misuse of the Xmas struct at compilation time
    pub fn crack(&mut self) -> Option<u64> {
        if self.encryption_weakness.is_some() {
            return self.encryption_weakness
        };

        if let Some(weakness) = self.weakness {
            // outer loop
            for i in 0..self.numbers.len() {
                let mut cursor = i;
                let mut sum = 0;
                let mut smallest = None;
                let mut largest = None;
                // "contiguous sum" loop
                while sum < weakness {
                    if let Some(next) = self.numbers.get(cursor) {
                        if smallest.is_none() || next < smallest.unwrap() {
                            smallest = Some(next);
                        }

                        if largest.is_none() || next > largest.unwrap() {
                            largest = Some(next);
                        }

                        sum += next;
                        cursor += 1;
                        if cursor - i >= 2 {
                            if sum == weakness {
                                self.encryption_weakness = Some(smallest.expect("must have a smallest by now") + largest.expect("must have a largest by now"));
                                return self.encryption_weakness
                            }
                        }
                    } else {
                        // println!("Run out of numbers: which is _probably_ a sign something has gone wrong");
                        break
                    }
                }
            }
            println!("Did not crack. I have failed.");
            None
        } else {
            println!("Could not crack because I have no weakness.");
            None
        }
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    // create an xmas with a preamble of length 25
    let mut xmas = Xmas::new(25);
    part1_inner(input, &mut xmas).expect("must detect weakness")
}

pub fn part1_inner(input: &str, xmas: &mut Xmas) -> Option<u64> {
    let ret = None;
    for line in input.lines() {
        let number = line.parse::<u64>().expect("input must be positive integers");
        if let Some(weakness) = xmas.process(number) {
            return Some(weakness);
        };
    };

    ret
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> Option<u64> {
    let mut xmas = Xmas::new(25);
    part2_inner(input, &mut xmas)
}

pub fn part2_inner(input: &str, mut xmas: &mut Xmas) -> Option<u64> {
    part1_inner(input, &mut xmas).expect("must detect weakness");
    xmas.crack()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part1_works() {
        // our test case needs a different preamble length, hence using part1_inner to share the functionality
        assert_eq!(127, part1_inner(TEST_INPUT, &mut Xmas::new(5)).expect("failed to find known weakness!"));
    }

    #[test]
    fn part2_works() {
        // our test case needs a different preamble length, hence using part1_inner to share the functionality
        assert_eq!(62, part2_inner(TEST_INPUT, &mut Xmas::new(5)).expect("failed to find known weakness!"));
    }
}
