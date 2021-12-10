use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct Reindeer {
	pub resting: bool,
	pub current_time: i32,
	pub distance: i32,
	pub points: i32,
	pub speed: i32,
	pub fly_time: i32,
	pub rest_time: i32,
}

pub fn parse_input(input: &str) -> Vec<Reindeer> {
	let mut reindeer = vec![];

	for line in input.lines() {
		let w = line.split(" ").collect::<Vec<&str>>();

		reindeer.push(Reindeer {
			resting: false,
			current_time: 0,
			distance: 0,
			points: 0,
			speed: w[3].parse::<i32>().unwrap(),
			fly_time: w[6].parse::<i32>().unwrap(),
			rest_time: w[13].parse::<i32>().unwrap(),
		});
	}

	reindeer
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
	let reindeer = parse_input(input);
	let mut max = 0;

	for r in reindeer {
		let mut distance = 0;
		let mut resting = false;
		let mut remaining_time = 2503;
		loop {
			if resting {
				if remaining_time <= r.rest_time {
					break;
				}
				remaining_time -= r.rest_time;
				resting = false;
			}
			else {
				if remaining_time <= r.fly_time {
					distance += remaining_time * r.speed;
					break;
				}
				remaining_time -= r.fly_time;
				distance += r.fly_time * r.speed;
				resting = true;
			}
		}
		if max < distance {
			max = distance;
		}
	}

	max
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
	let mut reindeer = parse_input(input);

	for _ in 0..2503 {
		let mut max = 0;
		for r in &mut reindeer {
			r.current_time += 1;
			if r.resting {
				if r.current_time == r.rest_time {
					r.current_time = 0;
					r.resting = false;
				}
			}
			else {
				r.distance += r.speed;
				if r.current_time == r.fly_time {
					r.current_time = 0;
					r.resting = true;
				}
			}
			if r.distance > max {
				max = r.distance;
			}
		}

		for mut r in &mut reindeer {
			if r.distance == max {
				r.points += 1;
			}
		}
	}

	let mut max = 0;
	for r in reindeer {
		if max < r.points {
			max = r.points;
		}
	}
	max
}
