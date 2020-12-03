#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(String, usize, usize, usize, usize)> {
	input
		.lines()
		.map(|i| {
			let mut l = i.replace("turn on ", "on,");
			l = l.replace("turn off ", "off,");
			l = l.replace("toggle ", "toggle,");
			l = l.replace(" through ", ",");
			let mut parts = l.split(",");
			(
				String::from(parts.next().unwrap()),
				parts.next().unwrap().parse().unwrap(),
				parts.next().unwrap().parse().unwrap(),
				parts.next().unwrap().parse().unwrap(),
				parts.next().unwrap().parse().unwrap(),
			)
		})
		.collect::<Vec<(String, usize, usize, usize, usize)>>()
}

#[aoc(day6, part1)]
pub fn part1(lines: &Vec<(String, usize, usize, usize, usize)>) -> i32 {
	let mut grid = [[false; 1000]; 1000];

	for (action, x1, y1, x2, y2) in lines.iter() {
		let x1 = *x1;
		let x2 = *x2;
		let y1 = *y1;
		let y2 = *y2;
		for x in x1..=x2 {
			for y in y1..=y2 {
				grid[x][y] = match action.as_str() {
					"on" => true,
					"off" => false,
					"toggle" => !grid[x][y],
					_ => unimplemented!(),
				}
			}
		}
	}

	let mut total = 0;
	for row in grid.iter() {
		for cell in row {
			if *cell {
				total += 1;
			}
		}
	}
	total
}

#[aoc(day6, part2)]
pub fn part2(lines: &Vec<(String, usize, usize, usize, usize)>) -> i32 {
	let mut grid = [[0; 1000]; 1000];

	for (action, x1, y1, x2, y2) in lines.iter() {
		let x1 = *x1;
		let x2 = *x2;
		let y1 = *y1;
		let y2 = *y2;
		for x in x1..=x2 {
			for y in y1..=y2 {
				match action.as_str() {
					"on" => grid[x][y] += 1,
					"off" => {
						if grid[x][y] != 0 {
							grid[x][y] -= 1;
						}
					},
					"toggle" => grid[x][y] += 2,
					_ => unimplemented!(),
				}
			}
		}
	}

	let mut total = 0;
	for row in grid.iter() {
		for cell in row {
			total += *cell;
		}
	}
	total
}
