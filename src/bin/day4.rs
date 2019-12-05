use std::{io, iter};
use std::io::prelude::*;

fn parse_input(s: &str) -> (u32, u32) {
	let mut args = s.split("-");
	let lo = args.next().unwrap().parse().unwrap();
	let hi = args.next().unwrap().parse().unwrap();
	(lo, hi)
}

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let (lo, hi) = parse_input(&input);

	let result1 = part_one(lo, hi);
	println!("Part One\n========\n\nThere are `{}` different passwords.\n", result1);

	let result2 = part_two(lo, hi);
	println!("Part Two\n========\n\nThere are `{}` different passwords.\n", result2);
}

fn digits(number: &u32) -> impl Iterator<Item = u8> {
	let mut state = *number;
	iter::from_fn(move || {
		if state == 0 {
			return None;
		}
		let next = (state % 10) as u8;
		state = state / 10;
		Some(next)
	})
}
fn pairs<I: Iterator>(mut lhs: I::Item, mut iter: I) -> impl Iterator<Item = (I::Item, I::Item)> where I::Item: Copy {
	iter::from_fn(move || {
		let item = (lhs, iter.next()?);
		lhs = item.1;
		Some(item)
	})
}

fn rule1(number: &u32) -> bool {
	let mut has_double_digit = false;

	for (prev_digit, cur_digit) in pairs(10, digits(number)) {
		if cur_digit > prev_digit {
			return false;
		}
		if cur_digit == prev_digit {
			has_double_digit = true;
		}
	}

	has_double_digit
}

fn rule2(number: &u32) -> bool {
	let mut has_double_digit = false;
	let mut repeat_count = 1;

	for (prev_digit, cur_digit) in pairs(10, digits(number)) {
		if cur_digit > prev_digit {
			return false;
		}
		if cur_digit == prev_digit {
			repeat_count += 1;
		}
		else {
			if repeat_count == 2 {
				has_double_digit = true;
			}
			repeat_count = 1;
		}
	}

	if repeat_count == 2 {
		has_double_digit = true;
	}

	has_double_digit
}

fn part_one(lo: u32, hi: u32) -> usize {
	(lo..=hi).filter(rule1).count()
}

fn part_two(lo: u32, hi: u32) -> usize {
	(lo..=hi).filter(rule2).count()
}

#[test]
fn test_rules1() {
	assert!(rule1(&122345));
	assert!(rule1(&111123));
	assert!(!rule1(&135679));
	assert!(rule1(&111111));
	assert!(!rule1(&223450));
	assert!(!rule1(&123789));

	assert!(rule1(&112233));
	assert!(rule1(&123444));
	assert!(rule1(&111122));
}

#[test]
fn test_rules2() {
	assert!(rule2(&122345));
	assert!(!rule2(&111123));
	assert!(!rule2(&135679));
	assert!(!rule2(&111111));
	assert!(!rule2(&223450));
	assert!(!rule2(&123789));

	assert!(rule2(&112233));
	assert!(!rule2(&123444));
	assert!(!rule2(&111234));
	assert!(rule2(&111122));
}
