use std::{cmp::min, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(i32, i32, i32)> {
	input
		.lines()
		.map(|line| {
			let mut values = line.split('x').map(|v| i32::from_str(v).unwrap());
			(values.next().unwrap(), values.next().unwrap(), values.next().unwrap())
		})
		.collect()
}

#[aoc(day2, part1)]
pub fn part1(gifts: &Vec<(i32, i32, i32)>) -> i32 {
	let mut total = 0;
	for (l, w, h) in gifts {
		let side1 = l * w;
		let side2 = w * h;
		let side3 = l * h;

		total += side1 * 2 + side2 * 2 + side3 * 2 + min(min(side1, side2), side3);
	}
	total
}

#[aoc(day2, part2)]
pub fn part2(gifts: &Vec<(i32, i32, i32)>) -> i32 {
	let mut total = 0;
	for (l, w, h) in gifts {
		let mut sides = [l, w, h];
		sides.sort();
		total += sides[0] * 2 + sides[1] * 2 + sides[0] * sides[1] * sides[2];
	}
	total
}
