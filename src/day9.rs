use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Location {
	pub name: String,
	pub links: HashMap<String, u32>,
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> HashMap<String, Location> {
	let mut pairs = vec![];
	let mut locations = HashMap::new();

	for line in input.lines() {
		let w = line.split(" = ").collect::<Vec<&str>>();
		let l = w[0].split(" to ").map(String::from).collect::<Vec<String>>();
		let d = w[1].parse::<u32>().unwrap();

		if !locations.contains_key(&l[0]) {
			locations.insert(l[0].clone(), Location {
				name: l[0].clone(),
				links: HashMap::new(),
			});
		}
		if !locations.contains_key(&l[1]) {
			locations.insert(l[1].clone(), Location {
				name: l[1].clone(),
				links: HashMap::new(),
			});
		}
		pairs.push((l[0].clone(), l[1].clone(), d));
	}

	for (l1, l2, d) in pairs {
		locations.get_mut(&l1).unwrap().links.insert(l2.clone(), d);
		locations.get_mut(&l2).unwrap().links.insert(l1.clone(), d);
	}

	locations
}

#[aoc(day9, part1)]
pub fn part1(locations: &HashMap<String, Location>) -> u32 {
	let mut shortest = u32::max_value();
	for c in locations.keys().map(String::from).permutations(locations.len()) {
		let mut total = 0;

		for (l1, l2) in c.iter().tuple_windows() {
			total += locations.get(l1).unwrap().links.get(l2).unwrap();
		}

		if total < shortest {
			shortest = total;
		}
	}

	shortest
}

#[aoc(day9, part2)]
pub fn part2(locations: &HashMap<String, Location>) -> u32 {
	let mut longest = 0;
	for c in locations.keys().map(String::from).permutations(locations.len()) {
		let mut total = 0;

		for (l1, l2) in c.iter().tuple_windows() {
			total += locations.get(l1).unwrap().links.get(l2).unwrap();
		}

		if total > longest {
			longest = total;
		}
	}

	longest
}
