// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration
#![deny(
	warnings,
	nonstandard_style,
	unused,
	future_incompatible,
	rust_2018_idioms,
	unsafe_code
)]
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate md5;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

aoc_lib! { year = 2015 }
