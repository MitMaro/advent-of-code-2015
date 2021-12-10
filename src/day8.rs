use aoc_runner_derive::{aoc, aoc_generator};

enum State {
	Normal,
	Escape,
	HexFirst,
	HexSecond,
}
#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
	input
		.lines()
		.map(|i| i.chars().skip(1).take(i.len() - 2).collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>()
}

#[aoc(day8, part1)]
pub fn part1(lines: &Vec<Vec<char>>) -> usize {
	let mut file_size = 0;
	let mut memory_size = 0;
	for line in lines {
		file_size += line.len() + 2;
		let mut state = State::Normal;
		for c in line {
			match state {
				State::Normal => {
					if c == &'\\' {
						state = State::Escape;
					}
					else {
						memory_size += 1;
					}
				},
				State::Escape => {
					if c == &'x' {
						state = State::HexFirst;
					}
					else if c == &'"' || c == &'\\' {
						state = State::Normal;
						memory_size += 1;
					}
					else {
						state = State::Normal;
						memory_size += 2;
					}
				},
				State::HexFirst => {
					state = State::HexSecond;
				},
				State::HexSecond => {
					state = State::Normal;
					memory_size += 1;
				},
			}
		}
	}
	file_size - memory_size
}

#[aoc(day8, part2)]
pub fn part2(lines: &Vec<Vec<char>>) -> usize {
	let mut original_size = 0;
	let mut new_size = 0;
	for line in lines {
		original_size += line.len() + 2;
		new_size += line.len() + 4 + 2; // start and end quotes escaped + new quotes
		for c in line {
			if c == &'\\' || c == &'"' {
				new_size += 1;
			}
		}
	}
	new_size - original_size
}
