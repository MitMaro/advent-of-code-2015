#[derive(Debug)]
pub struct Aunt {
	pub index: i32,
	pub nones: i32,
	pub children: Option<i32>,
	pub cats: Option<i32>,
	pub samoyeds: Option<i32>,
	pub pomeranians: Option<i32>,
	pub akitas: Option<i32>,
	pub vizslas: Option<i32>,
	pub goldfish: Option<i32>,
	pub trees: Option<i32>,
	pub cars: Option<i32>,
	pub perfumes: Option<i32>,
}

impl Aunt {
	pub fn new(index: i32) -> Self {
		Aunt {
			index,
			nones: 10,
			children: None,
			cats: None,
			samoyeds: None,
			pomeranians: None,
			akitas: None,
			vizslas: None,
			goldfish: None,
			trees: None,
			cars: None,
			perfumes: None,
		}
	}

	pub fn update_none_count(&mut self) {
		self.nones = if self.children.is_some() { 0 } else { 1 }
			+ if self.cats.is_some() { 0 } else { 1 }
			+ if self.samoyeds.is_some() { 0 } else { 1 }
			+ if self.pomeranians.is_some() { 0 } else { 1 }
			+ if self.akitas.is_some() { 0 } else { 1 }
			+ if self.vizslas.is_some() { 0 } else { 1 }
			+ if self.goldfish.is_some() { 0 } else { 1 }
			+ if self.trees.is_some() { 0 } else { 1 }
			+ if self.cars.is_some() { 0 } else { 1 }
			+ if self.perfumes.is_some() { 0 } else { 1 };
	}
}

pub fn parse_input(input: &str) -> Vec<Aunt> {
	let mut aunts = vec![];

	let mut index = 1;
	for line in input.lines() {
		let d = line.splitn(2, ": ").map(String::from).collect::<Vec<String>>();

		let properties = d[1].split(", ");

		let mut aunt = Aunt::new(index);
		index += 1;
		for prop in properties {
			let v = prop.split(": ").map(String::from).collect::<Vec<String>>();

			let value = v[1].parse::<i32>().unwrap();

			match v[0].as_str() {
				"children" => aunt.children = Some(value),
				"cats" => aunt.cats = Some(value),
				"samoyeds" => aunt.samoyeds = Some(value),
				"pomeranians" => aunt.pomeranians = Some(value),
				"akitas" => aunt.akitas = Some(value),
				"vizslas" => aunt.vizslas = Some(value),
				"goldfish" => aunt.goldfish = Some(value),
				"trees" => aunt.trees = Some(value),
				"cars" => aunt.cars = Some(value),
				"perfumes" => aunt.perfumes = Some(value),
				_ => unreachable!(),
			}

			aunt.update_none_count();
		}
		aunts.push(aunt);
	}
	aunts
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
	let mut aunts = parse_input(input);
	aunts.retain(|a| {
		if let Some(v) = a.children {
			if v != 3 {
				return false;
			}
		}

		if let Some(v) = a.cats {
			if v != 7 {
				return false;
			}
		}

		if let Some(v) = a.samoyeds {
			if v != 2 {
				return false;
			}
		}
		if let Some(v) = a.pomeranians {
			if v != 3 {
				return false;
			}
		}

		if let Some(v) = a.akitas {
			if v != 0 {
				return false;
			}
		}

		if let Some(v) = a.vizslas {
			if v != 0 {
				return false;
			}
		}

		if let Some(v) = a.goldfish {
			if v != 5 {
				return false;
			}
		}

		if let Some(v) = a.trees {
			if v != 3 {
				return false;
			}
		}

		if let Some(v) = a.cars {
			if v != 2 {
				return false;
			}
		}

		if let Some(v) = a.perfumes {
			if v != 1 {
				return false;
			}
		}

		true
	});

	aunts[0].index
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
	let mut aunts = parse_input(input);
	aunts.retain(|a| {
		if let Some(v) = a.children {
			if v != 3 {
				return false;
			}
		}

		if let Some(v) = a.cats {
			if v <= 7 {
				return false;
			}
		}

		if let Some(v) = a.samoyeds {
			if v != 2 {
				return false;
			}
		}
		if let Some(v) = a.pomeranians {
			if v >= 3 {
				return false;
			}
		}

		if let Some(v) = a.akitas {
			if v != 0 {
				return false;
			}
		}

		if let Some(v) = a.vizslas {
			if v != 0 {
				return false;
			}
		}

		if let Some(v) = a.goldfish {
			if v >= 5 {
				return false;
			}
		}

		if let Some(v) = a.trees {
			if v <= 3 {
				return false;
			}
		}

		if let Some(v) = a.cars {
			if v != 2 {
				return false;
			}
		}

		if let Some(v) = a.perfumes {
			if v != 1 {
				return false;
			}
		}

		true
	});

	aunts[0].index
}
