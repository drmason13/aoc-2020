use aoc_runner_derive::aoc;

fn check_slope(slope: (usize, usize), map: &Vec<&str>) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    let width = map[0].len();

    while y < map.len() {
        let is_tree = map[y].as_bytes()[x % width] == b'#';
        if is_tree {
            tree_count += 1;
        }
        x += slope.0;
        y += slope.1
    }
    tree_count
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let map = input.lines().collect();
    check_slope((3, 1), &map)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let map = input.lines().collect();

    let mut answer = 1;
    for slope in &[
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ] {
        answer *= check_slope(*slope, &map);
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 336);
    }
}