use aoc_runner_derive::aoc;

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
	let mut grid = input
		.lines()
		.map(|v| {
			let mut line = v.chars().map(|c| c == '#').collect::<Vec<bool>>();
			line.insert(0, false);
			line.push(false);
			line
		})
		.collect::<Vec<Vec<bool>>>();
	grid.insert(0, vec![false; grid[0].len()]);
	grid.push(vec![false; grid[0].len()]);
	grid
}

fn s(cell: &bool) -> i32 {
	if *cell {
		1
	}
	else {
		0
	}
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
	let mut grid = parse_input(input);

	let grid_width = grid[0].len() - 1;
	let grid_height = grid.len() - 1;

	for _ in 0..100 {
		let g = grid.clone();
		for r in 1..grid_height {
			for c in 1..grid_width {
				let neighbors = s(&g[r + 1][c - 1])
					+ s(&g[r + 1][c]) + s(&g[r + 1][c + 1])
					+ s(&g[r][c - 1]) + s(&g[r][c + 1])
					+ s(&g[r - 1][c - 1])
					+ s(&g[r - 1][c]) + s(&g[r - 1][c + 1]);
				grid[r][c] = if g[r][c] {
					neighbors == 2 || neighbors == 3
				}
				else {
					neighbors == 3
				}
			}
		}
	}

	grid.iter().flatten().map(s).sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> i32 {
	let mut grid = parse_input(input);

	let grid_width = grid[0].len() - 1;
	let grid_height = grid.len() - 1;

	for _ in 0..100 {
		grid[1][1] = true;
		grid[1][grid_width - 1] = true;
		grid[grid_height - 1][1] = true;
		grid[grid_height - 1][grid_width - 1] = true;
		let g = grid.clone();
		for r in 1..grid_height {
			for c in 1..grid_width {
				let neighbors = s(&g[r + 1][c - 1])
					+ s(&g[r + 1][c]) + s(&g[r + 1][c + 1])
					+ s(&g[r][c - 1]) + s(&g[r][c + 1])
					+ s(&g[r - 1][c - 1])
					+ s(&g[r - 1][c]) + s(&g[r - 1][c + 1]);
				grid[r][c] = if g[r][c] {
					neighbors == 2 || neighbors == 3
				}
				else {
					neighbors == 3
				}
			}
		}
	}
	grid[1][1] = true;
	grid[1][grid_width - 1] = true;
	grid[grid_height - 1][1] = true;
	grid[grid_height - 1][grid_width - 1] = true;

	grid.iter().flatten().map(s).sum()
}
