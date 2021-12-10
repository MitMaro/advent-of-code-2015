use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
	And,
	Or,
	RShift,
	LShift,
	Not,
	Assignment,
}

impl Operation {
	fn new(v: &str) -> Self {
		match v {
			"AND" => Self::And,
			"OR" => Self::Or,
			"RSHIFT" => Self::RShift,
			"LSHIFT" => Self::LShift,
			"NOT" => Self::Not,
			_ => Self::Assignment,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Register {
	value: Option<u16>,
	name: Option<String>,
}

impl Register {
	fn new(input: &str) -> Self {
		if let Ok(v) = input.parse::<u16>() {
			Self {
				value: Some(v),
				name: None,
			}
		}
		else {
			Self {
				value: None,
				name: Some(String::from(input)),
			}
		}
	}

	fn empty() -> Self {
		Self {
			value: None,
			name: None,
		}
	}

	fn has_value(&self) -> bool {
		self.value.is_some()
	}

	fn set_value(&mut self, v: u16) {
		self.value = Some(v);
	}
}

fn get_value_from_register(register: &Register, values: &HashMap<String, u16>) -> Option<u16> {
	if let Some(value) = &register.value {
		Some(*value)
	}
	else if let Some(name) = &register.name {
		if let Some(value) = values.get(name) {
			Some(*value)
		}
		else {
			None
		}
	}
	else {
		None
	}
}

#[derive(Clone, Debug)]
pub struct Connection {
	pub operation: Operation,
	pub output: Register,
	pub left: Register,
	pub right: Register,
}

pub fn input_parse(input: &str) -> Vec<Connection> {
	let mut connections = vec![];

	for line in input.lines() {
		let line_parts: Vec<&str> = line.split(" -> ").collect();
		let instruction = line_parts[0];
		let output_connection = line_parts[1];

		let instruction_parts: Vec<&str> = instruction.split(" ").collect();
		if instruction.contains("AND")
			|| instruction.contains("OR")
			|| instruction.contains("LSHIFT")
			|| instruction.contains("RSHIFT")
		{
			connections.push(Connection {
				output: Register::new(output_connection),
				left: Register::new(instruction_parts[0]),
				operation: Operation::new(instruction_parts[1]),
				right: Register::new(instruction_parts[2]),
			});
		}
		else if instruction.contains("NOT") {
			connections.push(Connection {
				output: Register::new(output_connection),
				left: Register::empty(),
				operation: Operation::new(instruction_parts[0]),
				right: Register::new(instruction_parts[1]),
			});
		}
		else {
			connections.push(Connection {
				output: Register::new(output_connection),
				left: Register::empty(),
				operation: Operation::new("Assignment"),
				right: Register::new(instruction_parts[0]),
			});
		}
	}
	connections
}

fn resolve(mut connections: Vec<Connection>) -> u16 {
	let mut values = HashMap::new();
	loop {
		let mut resolved = true;
		for connection in connections.iter_mut() {
			if connection.output.has_value() {
				continue;
			}

			resolved = false;

			let lhs = get_value_from_register(&connection.left, &values);
			let rhs = get_value_from_register(&connection.right, &values);

			match connection.operation {
				// binary operations
				Operation::And | Operation::Or | Operation::RShift | Operation::LShift => {
					if lhs.is_none() || rhs.is_none() {
						continue;
					}
					let lhs = lhs.unwrap();
					let rhs = rhs.unwrap();
					connection.output.set_value(match connection.operation {
						Operation::And => lhs & rhs,
						Operation::Or => lhs | rhs,
						Operation::RShift => lhs >> rhs,
						Operation::LShift => lhs << rhs,
						_ => {
							unreachable!()
						},
					})
				},
				// uniary
				Operation::Not => {
					if rhs.is_none() {
						continue;
					}
					connection.output.set_value(!rhs.unwrap());
				},
				// assignments
				Operation::Assignment => {
					if rhs.is_none() {
						continue;
					}
					connection.output.set_value(rhs.unwrap());
				},
			}

			if connection.output.has_value() && connection.output.name.is_some() {
				let name = connection.output.name.as_ref().unwrap();
				values.insert(name.clone(), connection.output.value.unwrap());
			}
		}

		if resolved {
			break;
		}
	}

	*values.get("a").unwrap()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u16 {
	resolve(input_parse(input))
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u16 {
	let mut connections = input_parse(input);
	let result_a = resolve(connections.clone());
	let b_assignment_index = connections
		.iter()
		.position(|c| {
			if let Some(name) = &c.output.name {
				name == "b"
			}
			else {
				false
			}
		})
		.unwrap();
	connections.remove(b_assignment_index);

	let mut b_register = Register::new("result_a");
	b_register.set_value(result_a);
	connections.push(Connection {
		output: Register::new("b"),
		left: Register::empty(),
		right: b_register,
		operation: Operation::new("Assigment"),
	});
	resolve(connections)
}
