use std::{fmt, io};
use std::io::prelude::*;
use std::str::FromStr;

//----------------------------------------------------------------

#[derive(Clone, Debug)]
struct ParseDirError;
impl From<std::num::ParseIntError> for ParseDirError {
	fn from(_: std::num::ParseIntError) -> Self {
		ParseDirError
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Step {
	Left(i32),
	Right(i32),
	Up(i32),
	Down(i32),
}
impl Step {
	fn value(self) -> i32 {
		match self {
			Step::Left(steps) => steps,
			Step::Right(steps) => steps,
			Step::Up(steps) => steps,
			Step::Down(steps) => steps,
		}
	}
}
impl FromStr for Step {
	type Err = ParseDirError;
	fn from_str(s: &str) -> Result<Step, ParseDirError> {
		if s.len() < 2 {
			return Err(ParseDirError);
		}
		let steps = s[1..].parse()?;
		match s.as_bytes().first() {
			Some(&b'L') => Ok(Step::Left(steps)),
			Some(&b'R') => Ok(Step::Right(steps)),
			Some(&b'U') => Ok(Step::Up(steps)),
			Some(&b'D') => Ok(Step::Down(steps)),
			_ => Err(ParseDirError),
		}
	}
}
impl fmt::Display for Step {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (dir, steps) = match self {
			Step::Left(steps) => ("L", steps),
			Step::Right(steps) => ("R", steps),
			Step::Up(steps) => ("U", steps),
			Step::Down(steps) => ("D", steps),
		};
		write!(f, "{}{}", dir, steps)
	}
}

fn parse_wire(line: &str) -> Result<Vec<Step>, ParseDirError> {
	line.split(",").map(|s| s.parse()).collect()
}

//----------------------------------------------------------------

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let mut lines = input.lines();
	let first = lines.next().expect("unexpected eos");
	let second = lines.next().expect("unexpected eos");

	let wire1 = parse_wire(first).expect("invalid wire");
	let wire2 = parse_wire(second).expect("invalid wire");

	let result1 = part_one(&wire1, &wire2);
	print!("Part One\n========\n\nThe wires cross closest to the central port with a manhattan distance of `{}`\n\n", result1);

	let result2 = part_two(&wire1, &wire2);
	print!("Part Two\n========\n\nThe wires cross first with a manhattan distance of `{}`\n\n", result2);
}

//----------------------------------------------------------------

/// Advances the wire in the given direction.
fn advance((x, y): (i32, i32), dir: Step) -> (i32, i32) {
	match dir {
		Step::Left(steps) => (x - steps, y),
		Step::Right(steps) => (x + steps, y),
		Step::Up(steps) => (x, y - steps),
		Step::Down(steps) => (x, y + steps),
	}
}
/// Intersects two pieces of wire.
/// Returns the intersection (x, y) if there is any.
fn intersect((x1, y1): (i32, i32), dir1: Step, (x2, y2): (i32, i32), dir2: Step) -> Option<(i32, i32)> {
	match (dir1, dir2) {
		(Step::Left(steps1), Step::Up(steps2)) => intersect_lines((x1, x1 - steps1, y1), (y2, y2 - steps2, x2)),
		(Step::Left(steps1), Step::Down(steps2)) => intersect_lines((x1, x1 - steps1, y1), (y2, y2 + steps2, x2)),

		(Step::Right(steps1), Step::Up(steps2)) => intersect_lines((x1, x1 + steps1, y1), (y2, y2 - steps2, x2)),
		(Step::Right(steps1), Step::Down(steps2)) => intersect_lines((x1, x1 + steps1, y1), (y2, y2 + steps2, x2)),

		(Step::Up(steps1), Step::Left(steps2)) => intersect_lines((x2, x2 - steps2, y2), (y1, y1 - steps1, x1)),
		(Step::Up(steps1), Step::Right(steps2)) => intersect_lines((x2, x2 + steps2, y2), (y1, y1 - steps1, x1)),

		(Step::Down(steps1), Step::Left(steps2)) => intersect_lines((x2, x2 - steps2, y2), (y1, y1 + steps1, x1)),
		(Step::Down(steps1), Step::Right(steps2)) => intersect_lines((x2, x2 + steps2, y2), (y1, y1 + steps1, x1)),

		// Both lines are vertical or horizontal and cannot intersect
		_ => None,
	}
}
/// Intersects a vertial and horizontal line segment.
/// Returns the intersection (x, y) if there is any.
fn intersect_lines((x1, x2, y): (i32, i32, i32), (y1, y2, x): (i32, i32, i32)) -> Option<(i32, i32)> {
	// Sort the points x1/y1 should be smaller than x2/y2
	let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
	let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
	// No intersection if they fall outside the boundaries
	if x <= x1 || x >= x2 || y <= y1 || y >= y2 {
		return None;
	}
	// Intersection found
	Some((x, y))
}

/// Find all intersections between the two wires.
fn intersections(wire1: &[Step], wire2: &[Step], f: &mut dyn FnMut(i32, i32)) {
	let mut point1 = (0, 0);
	let mut dist1 = 0;
	for &step1 in wire1 {
		let mut point2 = (0, 0);
		let mut dist2 = 0;
		for &step2 in wire2 {
			if let Some((x, y)) = intersect(point1, step1, point2, step2) {
				let extra1 = (point1.0 - x).abs() + (point1.1 - y).abs();
				let extra2 = (point2.0 - x).abs() + (point2.1 - y).abs();
				f(dist1 + dist2 + extra1 + extra2, x.abs() + y.abs());
			}
			point2 = advance(point2, step2);
			dist2 += step2.value();
		}
		point1 = advance(point1, step1);
		dist1 += step1.value();
	}
}

//----------------------------------------------------------------

fn part_one(wire1: &[Step], wire2: &[Step]) -> i32 {
	let mut dist = 999999;
	intersections(wire1, wire2, &mut |_, central_dist| {
		if central_dist < dist {
			dist = central_dist;
		}
	});
	dist
}

#[test]
fn example1a() {
	let wire1 = parse_wire("R8,U5,L5,D3").unwrap();
	let wire2 = parse_wire("U7,R6,D4,L4").unwrap();
	assert_eq!(6, part_one(&wire1, &wire2));
}
#[test]
fn example2a() {
	let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
	let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
	assert_eq!(159, part_one(&wire1, &wire2));
}
#[test]
fn example3a() {
	let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
	let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
	assert_eq!(135, part_one(&wire1, &wire2));
}

//----------------------------------------------------------------

fn part_two(wire1: &[Step], wire2: &[Step]) -> i32 {
	let mut dist = 999999;
	intersections(wire1, wire2, &mut |wire_dist, _| {
		if wire_dist < dist {
			dist = wire_dist;
		}
	});
	dist
}

#[test]
fn example1b() {
	let wire1 = parse_wire("R8,U5,L5,D3").unwrap();
	let wire2 = parse_wire("U7,R6,D4,L4").unwrap();
	assert_eq!(30, part_two(&wire1, &wire2));
}
#[test]
fn example2b() {
	let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
	let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
	assert_eq!(610, part_two(&wire1, &wire2));
}
#[test]
fn example3b() {
	let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
	let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
	assert_eq!(410, part_two(&wire1, &wire2));
}
