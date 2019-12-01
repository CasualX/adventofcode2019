use std::io;
use std::io::prelude::*;

fn calculate_recursive_fuel(mass: f64) -> f64 {
	let fuel = (mass / 3.0).floor() - 2.0;
	if fuel <= 0.0 {
		return 0.0;
	}
	fuel + calculate_recursive_fuel(fuel)
}

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let mass = input.lines().map(|line| line.trim().parse::<f64>().expect("error parsing mass number")).collect::<Vec<_>>();

	let total_fuel1 = mass.iter().cloned().map(|mass| (mass / 3.0).floor() - 2.0).sum::<f64>();

	println!("Part One\n========\n\nThe sum of the fuel requirements is `{}`.\n", total_fuel1);

	let total_fuel2 = mass.iter().cloned().map(calculate_recursive_fuel).sum::<f64>();

	println!("Part Two\n========\n\nThe sum of the fuel requirements is `{}`.\n", total_fuel2);
}
