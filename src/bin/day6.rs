use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

struct Orbit<'a> {
	object: &'a str,
	center: &'a str,
}
impl<'a> Orbit<'a> {
	fn from_str(s: &'a str) -> Orbit<'a> {
		let mut iter = s.split(")");
		let center = iter.next().expect(s);
		let object = iter.next().expect(s);
		Orbit { center, object }
	}
}

fn parse_orbits(s: &'_ str) -> Vec<Orbit<'_>> {
	s.trim().lines().map(|s| Orbit::from_str(s)).collect()
}

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let orbits = parse_orbits(&input);

	let orbit_count_checksum = part_one(&orbits);
	println!("Part One\n========\n\nThe orbit count checksum is `{}`.\n", orbit_count_checksum);

	let min_orbital_transfers = part_two(&orbits);
	println!("Part Two\n========\n\nMinimum number of orbital transfers is `{}`.\n", min_orbital_transfers);
}

fn part_one(orbits: &[Orbit<'_>]) -> usize {
	let mut objects = HashMap::new();
	let mut subjects = HashSet::new();

	for orbit in orbits {
		let _ = objects.insert(orbit.object, orbit.center);
		let _ = subjects.insert(orbit.object);
	}

	let mut count = 0;
	for &value in &subjects {
		let mut value = value;
		while let Some(&next_value) = objects.get(value) {
			value = next_value;
			count += 1;
		}
	}

	count
}

fn part_two(orbits: &[Orbit<'_>]) -> usize {
	let mut objects = HashMap::new();
	for orbit in orbits {
		let _ = objects.insert(orbit.object, orbit.center);
	}

	let mut a = parents(&objects, "YOU");
	let mut b = parents(&objects, "SAN");
	a.reverse();
	b.reverse();

	let mut prefix_len = 0;
	while a[prefix_len] == b[prefix_len] {
		prefix_len += 1;
	}

	a.len() + b.len() - prefix_len - prefix_len
}

fn parents<'a>(orbits: &HashMap<&'a str, &'a str>, mut node: &'a str) -> Vec<&'a str> {
	let mut parents = Vec::new();

	while let Some(&parent) = orbits.get(node) {
		parents.push(parent);
		node = parent;
	}

	parents
}

#[test]
fn testa() {
	let orbits = parse_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
	assert_eq!(42, part_one(&orbits));
}

#[test]
fn testb() {
	let orbits = parse_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
	assert_eq!(4, part_two(&orbits));
}
