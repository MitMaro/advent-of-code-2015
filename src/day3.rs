use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<char> {
	input.chars().collect::<Vec<char>>()
}

#[aoc(day3, part1)]
pub fn part1(directions: &Vec<char>) -> usize {
	let mut houses = HashMap::new();
	houses.insert((0, 0), 1);
	let mut x = 0;
	let mut y = 0;

	for direction in directions {
		match direction {
			'^' => y += 1,
			'v' => y -= 1,
			'>' => x += 1,
			'<' => x -= 1,
			_ => unreachable!(),
		}

		*houses.entry((x, y)).or_insert(0) += 1;
	}

	houses.keys().count()
}

#[aoc(day3, part2)]
pub fn part2(directions: &Vec<char>) -> usize {
	let mut houses = HashMap::new();
	houses.insert((0, 0), 1);
	let mut santa_x = 0;
	let mut santa_y = 0;
	let mut robot_x = 0;
	let mut robot_y = 0;

	for (index, direction) in directions.iter().enumerate() {
		if index % 2 == 0 {
			match direction {
				'^' => santa_y += 1,
				'v' => santa_y -= 1,
				'>' => santa_x += 1,
				'<' => santa_x -= 1,
				_ => unreachable!(),
			}
			*houses.entry((santa_x, santa_y)).or_insert(0) += 1;
		}
		else {
			match direction {
				'^' => robot_y += 1,
				'v' => robot_y -= 1,
				'>' => robot_x += 1,
				'<' => robot_x -= 1,
				_ => unreachable!(),
			}
			*houses.entry((robot_x, robot_y)).or_insert(0) += 1;
		}
	}

	houses.keys().count()
}
