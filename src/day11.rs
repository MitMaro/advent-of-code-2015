fn array_to_string(i: &Vec<u8>) -> String {
	let mut r = vec![];
	for c in i.iter().take(i.len() - 1).rev() {
		r.push(((c + b'a') as char).to_string());
	}
	r.join("")
}

fn validate_password(password: &Vec<u8>) -> bool {
	let mut chain = false;
	let mut pairs = 0;
	let mut second_last_char = 30;
	let mut last_char = 30;

	for p in password.iter().take(password.len() - 1).rev() {
		let p = *p;
		if p == b'i' || p == b'o' || p == b'l' {
			return false;
		}

		if (second_last_char + 2) == (last_char + 1) && (last_char + 1) == p {
			chain = true;
		}

		if last_char == p && p != second_last_char {
			pairs += 1;
		}

		second_last_char = last_char;
		last_char = p;
	}

	chain && pairs >= 2
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
	let mut values = vec![];
	for i in input.chars().rev() {
		values.push(i as u8 - b'a');
	}
	values.push(b'z' - b'a');

	loop {
		let mut i = 0;
		loop {
			values[i] += 1;
			if values[i] == 26 {
				values[i] = 0;
				i += 1;
				if i == values.len() - 1 {
					break;
				}
				continue;
			}
			break;
		}

		if i == values.len() - 1 {
			unreachable!();
		}
		if validate_password(&values) {
			break;
		}
	}

	array_to_string(&values)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
	let p = part1(input);
	part1(p.as_str())
}
