use std::cmp::max;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Ingredient {
	pub capacity: i32,
	pub durability: i32,
	pub flavor: i32,
	pub texture: i32,
	pub calories: i32,
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<Ingredient> {
	let mut ingredients = vec![];

	for line in input.lines() {
		let w = line
			.replace(":", "")
			.replace(",", "")
			.split(" ")
			.map(String::from)
			.collect::<Vec<String>>();
		ingredients.push(Ingredient {
			capacity: w[2].parse::<i32>().unwrap(),
			durability: w[4].parse::<i32>().unwrap(),
			flavor: w[6].parse::<i32>().unwrap(),
			texture: w[8].parse::<i32>().unwrap(),
			calories: w[10].parse::<i32>().unwrap(),
		});
	}

	ingredients
}

#[aoc(day15, part1)]
pub fn part1(ing: &Vec<Ingredient>) -> i32 {
	let mut max_score = 0;
	for a in 0..=100 {
		for b in 0..(100 - a) {
			for c in 0..(100 - a - b) {
				let d = 100 - (a + b + c);
				let capacity = max(
					ing[0].capacity * a + ing[1].capacity * b + ing[2].capacity * c + ing[3].capacity * d,
					0,
				);
				let durability = max(
					ing[0].durability * a + ing[1].durability * b + ing[2].durability * c + ing[3].durability * d,
					0,
				);
				let flavor = max(
					ing[0].flavor * a + ing[1].flavor * b + ing[2].flavor * c + ing[3].flavor * d,
					0,
				);
				let texture = max(
					ing[0].texture * a + ing[1].texture * b + ing[2].texture * c + ing[3].texture * d,
					0,
				);
				let score = capacity * durability * flavor * texture;

				if score > max_score {
					max_score = score;
				}
			}
		}
	}
	max_score
}

#[aoc(day15, part2)]
pub fn part2(ing: &Vec<Ingredient>) -> i32 {
	let mut max_score = 0;
	for a in 0..=100 {
		for b in 0..(100 - a) {
			for c in 0..(100 - a - b) {
				let d = 100 - (a + b + c);
				let capacity = max(
					ing[0].capacity * a + ing[1].capacity * b + ing[2].capacity * c + ing[3].capacity * d,
					0,
				);
				let durability = max(
					ing[0].durability * a + ing[1].durability * b + ing[2].durability * c + ing[3].durability * d,
					0,
				);
				let flavor = max(
					ing[0].flavor * a + ing[1].flavor * b + ing[2].flavor * c + ing[3].flavor * d,
					0,
				);
				let texture = max(
					ing[0].texture * a + ing[1].texture * b + ing[2].texture * c + ing[3].texture * d,
					0,
				);
				let calories = ing[0].calories * a + ing[1].calories * b + ing[2].calories * c + ing[3].calories * d;
				let score = capacity * durability * flavor * texture;

				if calories == 500 && score > max_score {
					max_score = score;
				}
			}
		}
	}
	max_score
}
