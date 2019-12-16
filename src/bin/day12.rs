#![allow(non_snake_case)]

use std::{fmt, io};
use std::io::prelude::*;

fn parse_line(s: &str) -> Moon {
	let mut iter = s[1..s.len() - 1].split(", ");

	let x = iter.next().unwrap();
	let x = x[2..].parse().unwrap();

	let y = iter.next().unwrap();
	let y = y[2..].parse().unwrap();

	let z = iter.next().unwrap();
	let z = z[2..].parse().unwrap();

	Moon::new([x, y, z])
}
fn parse_moons(s: &str) -> Moons {
	let mut lines = s.trim().lines();

	let Io = parse_line(lines.next().unwrap());
	let Europa = parse_line(lines.next().unwrap());
	let Ganymede = parse_line(lines.next().unwrap());
	let Callisto = parse_line(lines.next().unwrap());

	Moons { Io, Europa, Ganymede, Callisto }
}

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let moons = parse_moons(&input);

	let total_energy = part_one(moons);
	print!("Part One\n========\n\nThe total energy in the system is `{}`.\n\n", total_energy);

	let repeat_steps = part_two(moons);
	print!("Part Two\n========\n\nIt took `{:?}` steps to repeat the initial state.\n\n", repeat_steps);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Moon {
	position: [i32; 3],
	velocity: [i32; 3],
}
impl Moon {
	fn new(position: [i32; 3]) -> Moon {
		Moon { position, velocity: [0; 3] }
	}
	fn gravity(lhs: &mut Moon, rhs: &mut Moon) {
		for i in 0..3 {
			if lhs.position[i] < rhs.position[i] {
				lhs.velocity[i] += 1;
				rhs.velocity[i] -= 1;
			}
			else if lhs.position[i] > rhs.position[i] {
				lhs.velocity[i] -= 1;
				rhs.velocity[i] += 1;
			}
		}
	}
	fn velocity(&mut self) {
		self.position[0] += self.velocity[0];
		self.position[1] += self.velocity[1];
		self.position[2] += self.velocity[2];
	}
	fn energy(&self) -> i32 {
		let potential = self.position[0].abs() + self.position[1].abs() + self.position[2].abs();
		let kinetic = self.velocity[0].abs() + self.velocity[1].abs() + self.velocity[2].abs();
		potential * kinetic
	}
	fn equals(&self, rhs: &Moon, i: usize) -> bool {
		self.position[i] == rhs.position[i] && self.velocity[i] == rhs.velocity[i]
	}
}
impl fmt::Display for Moon {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
			self.position[0], self.position[1], self.position[2],
			self.velocity[0], self.velocity[1], self.velocity[2])
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Moons {
	Io: Moon,
	Europa: Moon,
	Ganymede: Moon,
	Callisto: Moon,
}
impl Moons {
	fn simulate(&mut self) {
		// Apply all gravity
		Moon::gravity(&mut self.Io, &mut self.Europa);
		Moon::gravity(&mut self.Io, &mut self.Ganymede);
		Moon::gravity(&mut self.Io, &mut self.Callisto);
		Moon::gravity(&mut self.Europa, &mut self.Ganymede);
		Moon::gravity(&mut self.Europa, &mut self.Callisto);
		Moon::gravity(&mut self.Ganymede, &mut self.Callisto);
		// Apply all velocity
		Moon::velocity(&mut self.Io);
		Moon::velocity(&mut self.Europa);
		Moon::velocity(&mut self.Ganymede);
		Moon::velocity(&mut self.Callisto);
	}
	fn total_energy(&self) -> i32 {
		self.Io.energy() + self.Europa.energy() + self.Ganymede.energy() + self.Callisto.energy()
	}
	fn equals(&self, rhs: &Moons, i: usize) -> bool {
		self.Io.equals(&rhs.Io, i) && self.Europa.equals(&rhs.Europa, i) && self.Ganymede.equals(&rhs.Ganymede, i) && self.Callisto.equals(&rhs.Callisto, i)
	}

	fn positions(&self, i: usize) -> [i32; 4] {
		[
			self.Io.position[i],
			self.Europa.position[i],
			self.Ganymede.position[i],
			self.Callisto.position[i],
		]
	}
	fn velocities(&self, i: usize) -> [i32; 4] {
		[
			self.Io.velocity[i],
			self.Europa.velocity[i],
			self.Ganymede.velocity[i],
			self.Callisto.velocity[i],
		]
	}
}
impl fmt::Display for Moons {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}\n{}\n{}\n{}\n", self.Io, self.Europa, self.Ganymede, self.Callisto)
	}
}

fn part_one(mut moons: Moons) -> i32 {
	for _ in 0..1000 {
		moons.simulate();
	}
	moons.total_energy()
}

#[test]
fn test1a() {
	let mut moons = Moons {
		Io: Moon::new([-8, -10, 0]),
		Europa: Moon::new([5, 5, 10]),
		Ganymede: Moon::new([2, -7, 3]),
		Callisto: Moon::new([9, -8, -3]),
	};

	for i in 0..100 {
		if i % 10 == 0 {
			print!("After {} steps:\n{}\n", i, moons);
		}
		moons.simulate();
	}
	print!("After 100 steps:\n{}\n", moons);

	assert_eq!(moons.total_energy(), 1940);
}

/*
Yeah I had to cheat this one and look it up...

X, Y and Z axes can be simulated independently.
Once repetition on each axis is found the repetition for all axes is the least common multiple.

For extra credit, this implementation is *super* fast.
The simulation step is SIMD optimized (given a willing autovectorizing compiler :)
*/

fn gravity(pos1: i32, pos2: &[i32; 4]) -> i32 {
	fn f(pos1: i32, pos2: i32) -> i32 {
		if pos1 < pos2 { 1 }
		else if pos1 > pos2 { -1 }
		else { 0 }
	}
	f(pos1, pos2[0]) + f(pos1, pos2[1]) + f(pos1, pos2[2]) + f(pos1, pos2[3])
}

fn simulate(pos: &mut [i32; 4], vel: &mut [i32; 4]) {
	vel[0] += gravity(pos[0], pos);
	vel[1] += gravity(pos[1], pos);
	vel[2] += gravity(pos[2], pos);
	vel[3] += gravity(pos[3], pos);
	pos[0] += vel[0];
	pos[1] += vel[1];
	pos[2] += vel[2];
	pos[3] += vel[3];
}

fn gcd(a: i64, b: i64) -> i64 {
	if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
	a * b / gcd(a, b)
}

fn repetition(mut pos: [i32; 4], mut vel: [i32; 4]) -> i64 {
	let oldpos = pos;
	let oldvel = vel;
	let mut steps = 0;
	loop {
		simulate(&mut pos, &mut vel);
		steps += 1;
		if oldpos == pos && oldvel == vel {
			break steps;
		}
	}
}

fn part_two(moons: Moons) -> i64 {
	let x = repetition(moons.positions(0), moons.velocities(0));
	let y = repetition(moons.positions(1), moons.velocities(1));
	let z = repetition(moons.positions(2), moons.velocities(2));
	lcm(lcm(x, y), z)
}
