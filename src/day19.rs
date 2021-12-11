use aoc_runner_derive::aoc;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Replacement {
	pub from: String,
	pub to: String,
}

#[derive(Debug)]
pub struct Cali {
	input: String,
	replacements: Vec<Replacement>,
}

pub fn parse_input(input: &str) -> Cali {
	let mut replacements = vec![];
	let mut last_line = false;

	for line in input.trim().lines() {
		if line == "" {
			last_line = true;
			continue;
		}
		if last_line {
			return Cali {
				input: String::from(line),
				replacements,
			};
		}
		let p = line.split(" => ").map(String::from).collect::<Vec<String>>();

		replacements.push(Replacement {
			from: p[0].clone(),
			to: p[1].clone(),
		});
	}
	unreachable!()
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
	let cali = parse_input(input);
	let mut results = vec![];

	for Replacement { from: k, to: v } in cali.replacements {
		let p = cali.input.match_indices(k.as_str());
		for (i, _) in p {
			let mut n = cali.input.clone();
			n.replace_range(i..(i + k.len()), v.as_str());
			if !results.contains(&n) {
				results.push(n);
			}
		}
	}

	results.len()
}

fn replace_all(mol: String, from: &str, to: &str) -> (String, i32) {
	let matches = mol.matches(from).count();
	let new = mol.replace(from, to);

	(new, matches as i32)
}

// This isn't stable and may not always result in a correct answer. This is a known issue with this problem.
// See: https://www.reddit.com/r/adventofcode/comments/3xhkeb/day_19_part_2_proof_that_everyones_posted/
// Essentially, for most of the provided inputs, a greedy approach will work. For my input, that is not the case
// but by using a randomized replacement of rules it will eventually solve.
#[aoc(day19, part2)]
pub fn part2(input: &str) -> i32 {
	let mut cali = parse_input(input);

	let mut rng = rand::thread_rng();

	let mut last_mol = String::new();
	loop {
		let mut steps = 0;
		let mut mol = cali.input.clone();
		cali.replacements.shuffle(&mut rng);
		loop {
			for r in &cali.replacements {
				let (m, s) = replace_all(mol.clone(), r.to.as_str(), r.from.as_str());

				if m.contains("e") && m != "e" {
					continue;
				}
				mol = m;
				steps += s;

				if mol == "e" {
					return steps;
				}
			}
			// no changes, so the program has stalled, try a different random sequence
			if last_mol == mol {
				break;
			}
			last_mol = mol.clone();
		}
	}
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE_PART1: &'static str = indoc! {"
		H => HO
		H => OH
		O => HH

		HOHOHO
	"};

	static EXAMPLE_PART2: &'static str = indoc! {"
		e => H
		e => O
		H => HO
		H => OH
		O => HH

		HOHOHO
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(EXAMPLE_PART1), 7);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(EXAMPLE_PART2), 4218);
	}
}
