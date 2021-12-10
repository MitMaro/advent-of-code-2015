use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
	input.lines().map(String::from).collect::<Vec<String>>()
}

#[aoc(day5, part1)]
pub fn part1(lines: &Vec<String>) -> i32 {
	let mut total = 0;
	'l: for line in lines {
		let mut double_found = false;
		let mut vowels = 0;
		let mut last_char = '\0';

		for c in line.chars() {
			match c {
				'a' | 'e' | 'i' | 'o' | 'u' => {
					vowels += 1;
				},
				_ => {},
			}
			if last_char == c {
				double_found = true;
			}
			match last_char {
				'a' if c == 'b' => continue 'l,
				'c' if c == 'd' => continue 'l,
				'p' if c == 'q' => continue 'l,
				'x' if c == 'y' => continue 'l,
				_ => {},
			}

			last_char = c;
		}
		if double_found && vowels >= 3 {
			total += 1;
		}
	}

	total
}

#[aoc(day5, part2)]
pub fn part2(lines: &Vec<String>) -> i32 {
	let mut total = 0;
	for line in lines {
		let mut double_found = false;
		let mut repeat_found = false;
		let mut last_char = '\0';
		let mut second_last_char = '\0';

		for (index, c) in line.chars().enumerate() {
			let mut inner_last_character = '\0';
			for d in line.chars().skip(index + 1) {
				if inner_last_character != '\0' && last_char != '\0' && d == c && inner_last_character == last_char {
					double_found = true;
					break;
				}
				inner_last_character = d;
			}
			if second_last_char == c {
				repeat_found = true;
			}

			second_last_char = last_char;
			last_char = c;
		}

		if double_found && repeat_found {
			total += 1;
		}
	}

	total
}
