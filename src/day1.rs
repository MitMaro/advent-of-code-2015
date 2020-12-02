#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<char> {
	input.chars().collect::<Vec<char>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<char>) -> usize {
	let mut floor = 0;
	for direction in input.iter() {
		floor = if *direction == '(' { floor + 1 } else { floor - 1 };
	}
	floor
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<char>) -> usize {
	let mut floor = 0;
	for (position, direction) in input.iter().enumerate() {
		floor = if *direction == '(' { floor + 1 } else { floor - 1 };
		if floor == -1 {
			return position + 1;
		}
	}
	unreachable!();
}
