use aoc_runner_derive::aoc;
use serde_json::Value;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
	let mut total = 0;
	let mut numbers = 0;
	let mut negative = false;

	for char in input.chars() {
		if char.is_ascii_digit() {
			numbers *= 10;
			numbers += char.to_digit(10).unwrap() as i32;
		}
		else if numbers != 0 {
			total += if negative { -numbers } else { numbers };
			numbers = 0;
			negative = false;
		}
		else if char == '-' {
			negative = true;
		}
	}

	total
}

pub fn count(v: &Value) -> i32 {
	match v {
		Value::Null => 0,
		Value::Bool(_) => 0,
		Value::Number(n) => n.as_i64().unwrap() as i32,
		Value::String(_) => 0,
		Value::Array(values) => values.iter().fold(0, |t, v| t + count(v)),
		Value::Object(obj) => {
			let mut total = 0;
			for value in obj.values() {
				if let Value::String(v) = value {
					if v == "red" {
						return 0;
					}
				}
				total += count(value)
			}
			total
		},
	}
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
	let v: Value = serde_json::from_str(input).unwrap();
	count(&v)
}
