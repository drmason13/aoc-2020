use aoc_runner_derive::{aoc, aoc_generator};

use std::ops::{Add, AddAssign, Sub, SubAssign};

use parse_display::{Display, FromStr};

#[derive(Clone, Debug, FromStr, Display, PartialEq)]
pub enum Action {
    #[display("N{0}")]
    North(usize),
    #[display("E{0}")]
    East(usize),
    #[display("S{0}")]
    South(usize),
    #[display("W{0}")]
    West(usize),
    #[display("F{0}")]
    Forward(usize),
    #[display("L{0}")]
    Left(usize),
    #[display("R{0}")]
    Right(usize),
}

#[derive(Clone, Debug, PartialEq)]
struct Waypoint(CompassCoord);

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint(CompassCoord::new(10, 1))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CompassCoord {
    x: isize,
    y: isize,
}

impl Default for CompassCoord {
    fn default() -> Self {
        CompassCoord::new(0, 0)
    }
}

impl CompassCoord {
    fn new(x: isize, y: isize) -> Self {
        CompassCoord { x, y }
    }

    fn north(value: usize) -> Self {
        CompassCoord {
            x: 0,
            y: value as isize,
        }
    }

    fn east(value: usize) -> Self {
        CompassCoord {
            x: value as isize,
            y: 0,
        }
    }

    fn south(value: usize) -> Self {
        CompassCoord {
            x: 0,
            y: -(value as isize),
        }
    }

    fn west(value: usize) -> Self {
        CompassCoord {
            x: -(value as isize),
            y: 0,
        }
    }
}

impl Add for CompassCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        CompassCoord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for CompassCoord {
    fn add_assign(&mut self, other: Self) {
        *self = CompassCoord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Direction(usize);

impl Direction {
    /// creates a direction with the value normalised to the range [0, 360)
    fn new(mut value: isize) -> Self {
        while value < 0 {
            value += 360;
        }
        while value >= 360 {
            value -= 360
        }
        Direction(value as usize)
    }
}

/// The default direction is East (=90), not Direction(0)!
impl Default for Direction {
    fn default() -> Self {
        Direction(90)
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % 360)
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, other: Self) {
        *self = Self((self.0 + other.0) % 360)
    }
}

impl Sub for Direction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Direction::new(self.0 as isize - other.0 as isize)
    }
}

impl SubAssign for Direction {
    fn sub_assign(&mut self, other: Self) {
        *self = Direction::new(self.0 as isize - other.0 as isize)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Ship {
    position: CompassCoord,
    direction: Direction,
}

impl Ship {
    fn _new(position: CompassCoord, direction: Direction) -> Self {
        Ship {
            position,
            direction,
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs()) as usize
    }

    fn update(&mut self, action: &Action, wp: Option<&mut Waypoint>) {
        if wp.is_some() {
            // part2 uses a waypoint
            let waypoint = wp.unwrap();
            dbg!(&waypoint);
            dbg!(&self);
            dbg!(&action);
            match action {
                Action::North(x) => (*waypoint).0 += CompassCoord::north(*x),
                Action::East(x) => (*waypoint).0 += CompassCoord::east(*x),
                Action::South(x) => (*waypoint).0 += CompassCoord::south(*x),
                Action::West(x) => (*waypoint).0 += CompassCoord::west(*x),
                Action::Forward(x) => (0..*x).for_each(|_| self.position += waypoint.0.clone()),
                // some trig for the rotations...
                /*
                    cos(x)  -sin(x)
                    sin(x)   cos(x)

                    x=0
                    1   0
                    0   1

                    x=90
                    0   -1
                    1   0

                    x=180
                    -1  0
                    0  -1

                    x=270
                    0   1
                    -1  0
                */
                Action::Left(x) => match x {
                    0 => {}
                    90 => *waypoint = Waypoint(CompassCoord::new(-waypoint.0.y, waypoint.0.x)),
                    180 => *waypoint = Waypoint(CompassCoord::new(-waypoint.0.x, -waypoint.0.y)),
                    270 => *waypoint = Waypoint(CompassCoord::new(waypoint.0.y, -waypoint.0.x)),
                    _ => unimplemented!("this angle of rotation is not supported!"),
                },
                // rotating right is just rotating left but with 90 and 270 swapped
                Action::Right(x) => match x {
                    0 => {}
                    90 => *waypoint = Waypoint(CompassCoord::new(waypoint.0.y, -waypoint.0.x)),
                    180 => *waypoint = Waypoint(CompassCoord::new(-waypoint.0.x, -waypoint.0.y)),
                    270 => *waypoint = Waypoint(CompassCoord::new(-waypoint.0.y, waypoint.0.x)),
                    _ => unimplemented!("this angle of rotation is not supported!"),
                },
            }
        } else {
            // part1 does not use a waypoint
            match action {
                Action::North(x) => self.position += CompassCoord::north(*x),
                Action::East(x) => self.position += CompassCoord::east(*x),
                Action::South(x) => self.position += CompassCoord::south(*x),
                Action::West(x) => self.position += CompassCoord::west(*x),
                Action::Forward(x) => match self.direction {
                    Direction(0) => self.position += CompassCoord::north(*x),
                    Direction(90) => self.position += CompassCoord::east(*x),
                    Direction(180) => self.position += CompassCoord::south(*x),
                    Direction(270) => self.position += CompassCoord::west(*x),
                    _ => unimplemented!("non-orthognal directions not yet supported!"),
                },
                Action::Left(x) => self.direction -= Direction(*x),
                Action::Right(x) => self.direction += Direction(*x),
            }
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| line.parse::<Action>().expect("valid input"))
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(list: &Vec<Action>) -> usize {
    let mut ship = Ship::default();
    for action in list {
        // part1 does not use a waypoint
        ship.update(action, None);
    }
    ship.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn part2(list: &Vec<Action>) -> usize {
    let mut ship = Ship::default();
    // part2 uses a waypoint
    let mut waypoint = Waypoint::default();
    for action in list {
        ship.update(action, Some(&mut waypoint));
    }
    ship.manhattan_distance()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn part1_works() {
        assert_eq!(part1(&input_generator(TEST_INPUT)), 25);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&input_generator(TEST_INPUT)), 286);
    }
}
