fn expand(input: &str) -> String {
	let mut output = vec![];
	let mut it = input.chars();
	let mut m = it.next().unwrap();
	let mut count = 1;

	loop {
		if let Some(n) = it.next() {
			if n == m {
				count += 1;
			}
			else {
				output.push(format!("{}", count));
				output.push(m.to_string());
				count = 1;
				m = n;
			}
		}
		else {
			output.push(format!("{}", count));
			output.push(m.to_string());
			break;
		}
	}

	output.join("")
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
	let mut i = String::from(input);
	for _ in 0..40 {
		i = expand(i.as_str());
	}
	i.len()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
	let mut i = String::from(input);
	for _ in 0..50 {
		i = expand(i.as_str());
	}
	i.len()
}
