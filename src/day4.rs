use aoc_runner_derive::aoc;
use md5::{Digest, Md5};

#[aoc(day4, part1)]
pub fn part1(key: &str) -> usize {
	let mut i = 0;

	loop {
		let mut hasher = Md5::new();
		hasher.update(format!("{}{}", key, i));
		let result = hasher.finalize();

		if result[0] == 0 && result[1] == 0 && result[2] < 0x10 {
			return i;
		}
		i += 1;
	}
}

#[aoc(day4, part2)]
pub fn part2(key: &str) -> usize {
	let mut i = 0;

	loop {
		let mut hasher = Md5::new();
		hasher.update(format!("{}{}", key, i));
		let result = hasher.finalize();

		if result[0] == 0 && result[1] == 0 && result[2] == 0 {
			return i;
		}
		i += 1;
	}
}
