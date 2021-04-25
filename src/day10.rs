use aoc_runner_derive::{aoc_generator, aoc};

use std::collections::HashMap;

const MAX_JUMP: u64 = 3;

/// generate a sorted list from the input
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
	let mut list = input.lines()
		.map(|n| n.parse::<u64>().expect("input must be positive integers"))
		.collect::<Vec<_>>();

	list.sort_unstable();
	list
}

#[aoc(day10, part1)]
pub fn part1(list: &Vec<u64>) -> usize {
	// I think this is how it gets sorted by default!
	assert!(list.first() < list.last());

	let mut current_voltage = 0;
	let mut jump_counts = HashMap::<u64, usize>::new();

	for n in list {
		match n - current_voltage {
			jump if jump <= MAX_JUMP => {
				// record the jump
				*jump_counts.entry(jump).or_insert(0) += 1;
				// increment the current_voltage for next time
				current_voltage = *n;
			},
			_ => panic!("could not connect next adaptor as the difference in voltage was too high!\nFrom: {}\nTo: {}", current_voltage, n),
		}
	}

	// add the final jump of 3 to the device's built in adaptor
	jump_counts.get(&1).expect("must have a 1") * (jump_counts.get(&3).expect("must have a 3") + 1)
}

#[aoc(day10, part2)]
pub fn part2(list: &Vec<u64>) -> u64 {
	let mut route_solver = RouteSolver::new(list);
	route_solver.solve()
}

/// counts valid jumps from a position in the list
pub fn count_jumps(index: usize, list: &Vec<u64>) -> u64 {
	let mut jump = 0;

	for i in index + 1..=index + MAX_JUMP as usize {
		if let Some(x) = list.get(i) {
			if x - list[index] <= MAX_JUMP {
				jump += 1;
			}
		} else {
			break
		}
	}
	jump
}

#[derive(Debug)]
pub struct RouteSolver {
	list: Vec<u64>,
	node_jumps: Vec<u64>,
	node_routes: Vec<u64>,
}

impl RouteSolver {
	fn new(input: &Vec<u64>) -> Self {
		let list = std::iter::once(0)
			.chain(input.iter().cloned())
			.collect::<Vec<_>>();

		let node_jumps = list.iter()
			.enumerate()
			.map(|(i, _)| count_jumps(i, &list))
			.collect();
		let mut node_routes = std::iter::repeat(0).take(list.len()).collect::<Vec<u64>>();
		node_routes[list.len() - 1] = 1;

		RouteSolver {
			list,
			node_jumps,
			node_routes,
		}
	}

	fn solve(&mut self) -> u64 {
		dbg!(&self.list);
		for i in (0..self.list.len()).rev() {
			let mut checks = self.node_jumps[i] as usize;
			dbg!(i, self.list[i], self.node_jumps[i], self.node_routes[i]);
			while checks > 0 {
				self.node_routes[i] += self.node_routes[i + checks];
				dbg!(i, self.node_jumps[i], self.list[i], self.node_routes[i]);
				checks -= 1;
			}
		}
		dbg!(&self);
		self.node_routes[0]
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const TEST_INPUT: &'static str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

	const TEST_INPUT2: &'static str = "\
16
10
15
5
1
11
7
19
6
12
4";

	fn _mini_list() -> Vec<u64> {
		vec![4, 5, 7, 9, 10]
	}

	fn long_list() -> Vec<u64> {
		vec![1, 4, 5, 6, 7, 9, 10, 11, 12, 14, 16, 17, 19, 22]
	}

	#[test]
	fn part1_works() {
		assert_eq!(220, part1(&input_generator(TEST_INPUT)));
		assert_eq!(35, part1(&input_generator(TEST_INPUT2)));
	}

	#[test]
	fn route_solver_inits() {
		let long_list = long_list();
		let route_solver = RouteSolver::new(&long_list);

		//								   vec![0, 1, 4, 5, 6, 7, 9, 10,11,12,14,16,17,19,22]
		assert_eq!(route_solver.node_jumps, vec![1, 1, 3, 2, 2, 2, 3, 2, 2, 1, 2, 2, 1, 1, 0]);
		assert_eq!(route_solver.node_routes, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
	}

	#[test]
	#[ignore]
	fn route_solver_contiguous_lists() {
		let mut list = vec![0, 1, 2];
		for x in 2..100 {
			list.push(x);
			let mut route_solver = RouteSolver::new(&list);
			let out = route_solver.solve();
			println!("{}: {:#?}", x, out);
		}
	}

	#[test]
	fn route_solver_works() {
		let long_list = long_list();
		let mut route_solver = RouteSolver::new(&long_list);

		assert_eq!(144, route_solver.solve());
	}

	#[test]
	fn part2_works() {
		assert_eq!(8, part2(&input_generator(TEST_INPUT2)));
		assert_eq!(19208, part2(&input_generator(TEST_INPUT)));
	}
}
