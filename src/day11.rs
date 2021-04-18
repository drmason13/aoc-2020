use aoc_runner_derive::{aoc_generator, aoc};

use parse_display::{Display, FromStr};

use std::convert::TryFrom;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
	cells: HashMap<Coord, Cell>,
	width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
struct Coord {
	x: usize,
	y: usize,
}

impl Coord {
	/// returns an iterator of Cell that visits all adjacent cells that are valid (max 8)
	/// width is required to calculate cells that are out of bounds (assuming a square Grid)
	fn neighbours(&self, width: usize, height: usize) -> Neighbours {
		Neighbours::new(self, width, height)
	}

	/// attempts to offset this coordinate by dx and dy, returns None if this would make a
	/// coordinate outside of the bounds (0, 0), (width, height)
	fn offset(&self, dx: isize, dy: isize, width: usize, height: usize) -> Option<Coord> {
		let x = self.x as isize + dx;
		let y = self.y as isize + dy;
		if x < 0 || x >= width as isize {
			return None
		}
		if y < 0 || y >= height as isize {
			return None
		}
		Some(Coord { x: x as usize, y: y as usize })
	}
}

struct Neighbours {
	index: usize,
	coords: [Option<Coord>; 8],
}

impl Neighbours {
	fn new(coord: &Coord, width: usize, height: usize) -> Self {
		let coords = [
			coord.offset(-1, -1, width, height),
			coord.offset( 0, -1, width, height),
			coord.offset( 1, -1, width, height),
			coord.offset(-1,  0, width, height),
			coord.offset( 1,  0, width, height),
			coord.offset(-1,  1, width, height),
			coord.offset( 0,  1, width, height),
			coord.offset( 1,  1, width, height),
		];

		Neighbours {
			index: 0,
			coords,
		}
	}
}

// TODO: make a similar iterator Struct for the "visible niehgbours" to solve part 2
impl Iterator for Neighbours {
	type Item = Coord;

	fn next(&mut self) -> Option<Self::Item> {
		// increment index
		if self.index >= 8 {
			return None
		}

		let next = self.coords[self.index].as_ref();
		self.index += 1;

		if next.is_some() {
			next.cloned()
		} else {
			self.next()
		}
	}
}

impl Grid {
	/// returns count of occupied neighbours (each cell has up to 8 neighbours)
	fn occupied_neighbours(&self, coord: &Coord) -> usize {
		coord
			.neighbours(self.width, self.width)
			.filter_map(|coord| match self.cells.get(&coord) {
				Some(&Cell::Occupied) => Some(Cell::Occupied),
				_ => None,
			})
			.count()
	}

	fn next_generation(&mut self) {
		self.cells = self.cells.iter().map(|(coord, cell)| {
			let occupied_neighbours = self.occupied_neighbours(coord);
			let cell = match cell {
				Cell::Floor => Cell::Floor,
				Cell::Empty => if occupied_neighbours == 0 { Cell::Occupied } else { Cell::Empty },
				Cell::Occupied => if occupied_neighbours >= 4 { Cell::Empty } else { Cell::Occupied },
			};
			(coord.clone(), cell)
		}).collect();
	}
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for y in 0..self.width {
			for x in 0..self.width {
				write!(f, "{}", self.cells.get(&Coord { x, y }).unwrap())?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

#[derive(Clone, Display, FromStr, PartialEq, Debug)]
pub enum Cell {
	#[display("L")]
	Empty,
	#[display("#")]
	Occupied,
	#[display(".")]
	Floor,
}

impl TryFrom<char> for Cell {
	type Error = &'static str;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'L' => Ok(Cell::Empty),
			'#' => Ok(Cell::Occupied),
			'.' => Ok(Cell::Floor),
			_ => Err("invalid Cell char"),
		}
	}
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Grid {
	let mut width = 0;
	let cells = input
		.lines()
		.enumerate()
		.map(|(y, line)| {
			if width == 0 {
				// read width once during iteration (ugh)
				width = line.len();
			}
			line
				.chars()
				.enumerate()
				.map(|(x, c)| (Coord { x, y }, Cell::try_from(c).expect("invalid input")))
				.collect::<Vec<_>>()
		})
		.flatten()
		.collect();

	Grid { cells, width }
}

#[aoc(day11, part1)]
pub fn part1(grid: &Grid) -> usize {
	let mut grid = grid.clone();
    let mut prev = HashMap::<Coord, Cell>::new();

    while grid.cells != prev {
        prev = grid.cells.clone();
        grid.next_generation();
    }

    grid.cells.iter().filter(|(_coord, cell)| **cell == Cell::Occupied).count()
}

#[aoc(day11, part2)]
pub fn part2(list: &Grid) -> Result<usize, &'static str> {
	unimplemented!("")
}

#[cfg(test)]
mod test {
	use super::*;

	fn test_generation(n: usize) -> &'static str {
		match n {
			0 => "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
			1 => "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
			2 => "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
			3 => "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
			4 => "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
			5 => "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
			_ => "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
		}
	}

	#[test]
	fn parser_works() {
		let input = "\
#.L
#L.";

		let parsed = input_generator(input);

		assert_eq!(parsed.cells.get(&Coord { x: 0, y: 0 }), Some(&Cell::Occupied));
		assert_eq!(parsed.cells.get(&Coord { x: 1, y: 0 }), Some(&Cell::Floor));
		assert_eq!(parsed.cells.get(&Coord { x: 2, y: 0 }), Some(&Cell::Empty));
		assert_eq!(parsed.cells.get(&Coord { x: 0, y: 1 }), Some(&Cell::Occupied));
		assert_eq!(parsed.cells.get(&Coord { x: 1, y: 1 }), Some(&Cell::Empty));
		assert_eq!(parsed.cells.get(&Coord { x: 2, y: 1 }), Some(&Cell::Floor));
	}

    #[test]
    fn neighbours_works() {
        let grid = input_generator(test_generation(1));
        
		println!("grid:\n{}", grid);
        let mut neighbours = (Coord {x: 6, y: 0}).neighbours(grid.width, grid.width);

        assert_eq!(neighbours.next(), Some(Coord {x: 5, y: 0}));
        assert_eq!(neighbours.next(), Some(Coord {x: 7, y: 0}));
        assert_eq!(neighbours.next(), Some(Coord {x: 5, y: 1}));
        assert_eq!(neighbours.next(), Some(Coord {x: 6, y: 1}));
        assert_eq!(neighbours.next(), Some(Coord {x: 7, y: 1}));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn occupied_neighbours_works() {
        let grid = input_generator(test_generation(1));
        
		println!("grid:\n{}", grid);

        assert_eq!(grid.occupied_neighbours(&Coord { x: 6, y: 0}), 3);
    }

	#[test]
	fn generations_work() {
		let mut grid = input_generator(test_generation(0));
		for generation in 1..=6 {
			dbg!(generation);
			grid.next_generation();
			println!("actual:\n{}", grid);
			println!("expected:\n{}", test_generation(generation));

			assert_eq!(input_generator(test_generation(generation)), grid);
		}
	}

	#[test]
	fn part1_works() {
		assert_eq!(part1(&input_generator(test_generation(0))), 37);
	}

	#[test]
	#[ignore]
	fn part2_works() {
		unimplemented!("")
	}
}
