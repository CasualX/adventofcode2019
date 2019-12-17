use std::{cmp, io};
use std::collections::HashMap;
use std::io::prelude::*;

fn parse_line(line: &str) -> Vec<(i64, &str)> {
	fn item(s: &str) -> (i64, &str) {
		let mut c = s.trim().split(" ");
		let n = c.next().unwrap().parse().unwrap();
		(n, c.next().unwrap())
	}
	line.split("=>").map(|s| s.trim().split(",").map(item)).flatten().collect()
}

fn parse(s: &str) -> Inventory {
	let mut map = HashMap::new();
	let mut names = Vec::new();

	for line in s.lines() {
		let parsed = parse_line(line);
		for (_, name) in parsed {
			if !map.contains_key(name) {
				map.insert(name, names.len());
				names.push(String::from(name));
			}
		}
	}

	let mut matrix = Vec::new();
	let empty = [0; 64];
	for line in s.lines() {
		let offset = matrix.len();
		matrix.extend_from_slice(&empty[..names.len()]);
		let parsed = parse_line(line);
		let last = parsed.last().unwrap().1;
		for (value, name) in parsed {
			let index = map[name];
			matrix[offset + index] = if name == last { value } else { -value };
		}
	}

	Inventory { matrix, names }
}

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).expect("error reading input");

	let inv = parse(&input);

	let min_fuel = part_one(&inv, 1);
	print!("Part One\n========\n\nThe minimum ORE required to produce 1 FUEL is `{}`\n\n", min_fuel);

	let max_fuel = part_two(&inv);
	print!("Part Two\n========\n\nProduced `{}` FUEL from `{}` ORE.\n\n", max_fuel.1, max_fuel.0);
}

#[derive(Clone, Debug)]
pub struct Inventory {
	matrix: Vec<i64>,
	names: Vec<String>,
}
impl Inventory {
	pub fn width(&self) -> usize {
		self.names.len()
	}
	pub fn height(&self) -> usize {
		self.matrix.len() / self.width()
	}
	pub fn reaction(&self, id: usize) -> &[i64] {
		let index = self.width() * id;
		&self.matrix[index..index + self.width()]
	}
	pub fn reactions(&self) -> impl Iterator<Item = &[i64]> {
		(0..self.height()).map(move |id| self.reaction(id))
	}
	pub fn chemical_name(&self, chemical: usize) -> &str {
		&self.names[chemical]
	}
	pub fn find_chemical(&self, name: &str) -> usize {
		self.names.iter()
			.position(|rx_name| name == rx_name)
			.expect("unable to find chemical")
	}
	pub fn find_reaction(&self, output: usize) -> &[i64] {
		self.reactions()
			.find(|rx| rx[output] > 0)
			.expect("unable to find reaction")
	}
}

fn add(work: &mut [i64], rx: &[i64], times: i64) {
	let len = cmp::min(work.len(), rx.len());
	for i in 0..len {
		work[i] += rx[i] * times;
	}
}
fn is(work: &[i64], rx: &[i64]) -> Option<i64> {
	let len = cmp::min(work.len(), rx.len());
	let mut times = 999999999999;
	for i in 0..len {
		if rx[i] < 0 {
			if work[i] < -rx[i] {
				return None;
			}
			times = cmp::min(times, work[i] / -rx[i]);
		}
	}
	return Some(times);
}

fn part_one(inv: &Inventory, fuel: i64) -> i64 {
	let fuel_index = inv.find_chemical("FUEL");
	let ore_index = inv.find_chemical("ORE");

	let mut work = vec![0; inv.width()];
	work[fuel_index] = -fuel; // We want to produce X FUEL

	let mut done = false;
	while !done {
		done = true;

		for i in 0..inv.width() {
			if i == ore_index {
				continue;
			}

			if work[i] < 0 {
				done = false;
				let rx = inv.find_reaction(i);
				let mut n = -work[i] / rx[i];
				if -work[i] % rx[i] != 0 {
					n += 1;
				}
				// println!("i:{} work[i]:{} rx[i]:{} n:{}", i, work[i], rx[i], n);
				add(&mut work, rx, n);
				// while work[i] < 0 {
				// 	add(&mut work, rx, 1);
				// }
				// println!("add {}: {:?}", inv.names[i], work);
			}
		}
	}

	return -work[ore_index];
}

fn part_two(inv: &Inventory) -> (i64, i64) {
	let trillion = 1000000000000;

	let mut fuel = 1;
	while part_one(inv, fuel) < trillion {
		fuel = fuel + fuel;
		// println!("fuel: {}", fuel);
	}

	let mut low = fuel / 2;
	let mut high = fuel;
	while low + 1 != high {
		let middle = (low + high) / 2;
		if part_one(inv, middle) < trillion {
			low = middle;
		}
		else {
			high = middle;
		}
	}

	return (part_one(inv, low), low);

	// println!("low: {} high: {} fuel: {}", low, high, part_one(inv, high));

	// unimplemented!()

	// let fuel_index = inv.find_chemical("FUEL");
	// let ore_index = inv.find_chemical("ORE");

	// let mut work = vec![0; inv.width()];
	// work[ore_index] = 1000000000000; // We have one trillion ORE

	// let mut done = false;
	// while !done {
	// 	done = true;

	// 	for rx in inv.reactions() {
	// 		println!("trying {:?}", rx);
	// 		while let Some(times) = is(&work, rx) {
	// 			add(&mut work, rx, times);
	// 			eprintln!("work: {:?}", work);
	// 			done = false;
	// 		}
	// 	}

	// }

	// return work[fuel_index];
}

#[test]
fn test1() {
	let inv = Inventory {
		matrix: vec![
			-10, 10,  0,  0,  0,  0, 0,
			 -1,  0,  1,  0,  0,  0, 0,
			  0, -7, -1,  1,  0,  0, 0,
			  0, -7,  0, -1,  1,  0, 0,
			  0, -7,  0,  0, -1,  1, 0,
			  0, -7,  0,  0,  0, -1, 1,
		],
		names: vec![
			"ORE".into(),
			"A".into(),
			"B".into(),
			"C".into(),
			"D".into(),
			"E".into(),
			"FUEL".into(),
		],
	};
	assert_eq!(part_one(&inv, 1), 31);
}
#[test]
fn test2() {
	let inv = parse("\
		9 ORE => 2 A
		8 ORE => 3 B
		7 ORE => 5 C
		3 A, 4 B => 1 AB
		5 B, 7 C => 1 BC
		4 C, 1 A => 1 CA
		2 AB, 3 BC, 4 CA => 1 FUEL");
	assert_eq!(part_one(&inv, 1), 165);
}
#[test]
fn test3() {
	let inv = parse("\
		157 ORE => 5 NZVS
		165 ORE => 6 DCFZ
		44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
		12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
		179 ORE => 7 PSHF
		177 ORE => 5 HKGWZ
		7 DCFZ, 7 PSHF => 2 XJWVT
		165 ORE => 2 GPVTF
		3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
	assert_eq!(part_one(&inv, 1), 13312);
	assert_eq!(part_two(&inv).1, 82892753);
}
#[test]
fn test4() {
	let inv = parse("\
		2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
		17 NVRVD, 3 JNWZP => 8 VPVL
		53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
		22 VJHF, 37 MNCFX => 5 FWMGM
		139 ORE => 4 NVRVD
		144 ORE => 7 JNWZP
		5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
		5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
		145 ORE => 6 MNCFX
		1 NVRVD => 8 CXFTF
		1 VJHF, 6 MNCFX => 4 RFSQX
		176 ORE => 6 VJHF");
	assert_eq!(part_one(&inv, 1), 180697);
	assert_eq!(part_two(&inv).1, 5586022);
}
#[test]
fn test5() {
	let inv = parse("\
		171 ORE => 8 CNZTR
		7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
		114 ORE => 4 BHXH
		14 VRPVC => 6 BMBT
		6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
		6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
		15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
		13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
		5 BMBT => 4 WPTQ
		189 ORE => 9 KTJDG
		1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
		12 VRPVC, 27 CNZTR => 2 XDBXC
		15 KTJDG, 12 BHXH => 5 XCVML
		3 BHXH, 2 VRPVC => 7 MZWV
		121 ORE => 7 VRPVC
		7 XCVML => 6 RJRHP
		5 BHXH, 4 VRPVC => 5 LTCX");
	assert_eq!(part_one(&inv, 1), 2210736);
	assert_eq!(part_two(&inv).1, 460664);
}
