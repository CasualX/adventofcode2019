use std::io;
use std::io::prelude::*;

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let mut program = input.split(",")
		.map(|value| value.parse::<u32>().expect("error parsing opcode"))
		.collect::<Vec<u32>>();

	program[1] = 12;
	program[2] = 2;

	let result1 = execute(&mut program.clone());

	print!("Part One\n========\n\nResult: `{}`\n\n", result1);

	print!("Part Two\n========\n\n");
	let (noun, verb, result2) = find_inputs(&program);
	print!("Noun: `{}`\nVerb: `{}`\nResult: `{}`\n", noun, verb, result2);
}

//----------------------------------------------------------------
// Part One

fn execute(program: &mut [u32]) -> u32 {
	let mut ip = 0;
	loop {
		match program[ip] {
			1 => {
				let a = program[ip + 1] as usize;
				let b = program[ip + 2] as usize;
				let c = program[ip + 3] as usize;
				program[c] = program[a] + program[b];
				ip += 4;
			},
			2 => {
				let a = program[ip + 1] as usize;
				let b = program[ip + 2] as usize;
				let c = program[ip + 3] as usize;
				program[c] = program[a] * program[b];
				ip += 4;
			},
			99 => break,
			opcode => panic!("invalid opcode: {}", opcode),
		}
	}
	program[0]
}

#[test]
fn test1() {
	let mut program = [1,0,0,0,99];
	assert_eq!(2, execute(&mut program));
	assert_eq!(program, [2,0,0,0,99]);
}
#[test]
fn test2() {
	let mut program = [2,3,0,3,99];
	assert_eq!(2, execute(&mut program));
	assert_eq!(program, [2,3,0,6,99]);
}
#[test]
fn test3() {
	let mut program = [2,4,4,5,99,0];
	assert_eq!(2, execute(&mut program));
	assert_eq!(program, [2,4,4,5,99,9801]);
}
#[test]
fn test4() {
	let mut program = [1,1,1,4,99,5,6,0,99];
	assert_eq!(30, execute(&mut program));
	assert_eq!(program, [30,1,1,4,2,5,6,0,99]);
}

//----------------------------------------------------------------
// Part Two

fn find_inputs(program: &[u32]) -> (u32, u32, u32) {
	for noun in 0..=99 {
		for verb in 0..=99 {
			let mut program = program.to_vec();
			program[1] = noun;
			program[2] = verb;
			let result = execute(&mut program);
			if result == 19690720 {
				return (noun, verb, 100 * noun + verb);
			}
		}
	}
	panic!("No solutions found!");
}
