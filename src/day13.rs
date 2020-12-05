use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Person {
	pub name: String,
	pub opinions: HashMap<String, i32>,
}

pub fn parse_input(input: &str) -> HashMap<String, Person> {
	let mut people = HashMap::new();

	for line in input.lines() {
		let w = line.split(" ").collect::<Vec<&str>>();
		let p1 = String::from(w[0]);
		let value = if w[2] == "gain" { 1 } else { -1 } * w[3].parse::<i32>().unwrap();
		let p2 = String::from(w[10]).replace(".", "");

		if !people.contains_key(&p1) {
			let mut opinions = HashMap::new();
			opinions.insert(p2.clone(), value);
			people.insert(p1.clone(), Person {
				name: p1.clone(),
				opinions,
			});
		}
		else {
			people.get_mut(&p1).unwrap().opinions.insert(p2.clone(), value);
		}
	}

	people
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
	let persons = parse_input(input);
	let mut best = 0;
	for mut c in persons.keys().map(String::from).permutations(persons.len()) {
		let mut total = 0;

		c.push(c[0].clone());

		for (p1, p2) in c.iter().tuple_windows() {
			total += persons.get(p1).unwrap().opinions.get(p2).unwrap();
			total += persons.get(p2).unwrap().opinions.get(p1).unwrap();
		}

		if total > best {
			best = total;
		}
	}

	best
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i32 {
	let mut persons = parse_input(input);
	let mut opinions = HashMap::new();

	for c in persons.keys().map(String::from).collect::<Vec<String>>() {
		opinions.insert(c.clone(), 0);
		persons
			.get_mut(&c)
			.unwrap()
			.opinions
			.insert(String::from("XXXXXXXX"), 0);
	}
	persons.insert(String::from("XXXXXXXX"), Person {
		name: String::from("XXXXXXXX"),
		opinions,
	});

	let mut best = 0;
	for mut c in persons.keys().map(String::from).permutations(persons.len()) {
		let mut total = 0;

		c.push(c[0].clone());

		for (p1, p2) in c.iter().tuple_windows() {
			total += persons.get(p1).unwrap().opinions.get(p2).unwrap();
			total += persons.get(p2).unwrap().opinions.get(p1).unwrap();
		}

		if total > best {
			best = total;
		}
	}

	best
}
