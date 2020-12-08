use itertools::Itertools;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Vec<i32> {
	input.lines().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

#[aoc(day17, part1)]
pub fn part1(containers: &Vec<i32>) -> i32 {
	let mut total = 0;
	for l in 1..containers.len() {
		for c in containers.iter().combinations(l) {
			let t = c.iter().map(|v| **v).sum::<i32>();
			if t == 150 {
				total += 1;
			}
		}
	}
	total
}

#[aoc(day17, part2)]
pub fn part2(containers: &Vec<i32>) -> i32 {
	for l in 1..containers.len() {
		let mut total = 0;
		for c in containers.iter().combinations(l) {
			let t = c.iter().map(|v| **v).sum::<i32>();
			if t == 150 {
				total += 1;
			}
		}
		if total > 0 {
			return total;
		}
	}
	unreachable!();
}
