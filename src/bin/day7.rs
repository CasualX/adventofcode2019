use std::io;
use std::io::prelude::*;
use itertools::Itertools;

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let program = input.split(",")
		.map(|value| value.parse::<i32>().expect("error parsing opcode"))
		.collect::<Vec<i32>>();

	let result1 = part_one(&program);
	print!("Part One\n========\n\nThe computer prints the diagnostic value `{}`.\n\n", result1);

	// let result2 = execute(&mut program, 5);
	// print!("Part Two\n========\n\nThe computer prints the diagnostic value `{}`.\n\n", result2);
}


#[derive(Copy, Clone, Debug)]
enum Arg {
	Position(i32),
	Immediate(i32),
}
impl Arg {
	fn decode(imm: bool, arg: i32) -> Arg {
		match imm {
			false => Arg::Position(arg),
			true => Arg::Immediate(arg),
		}
	}
}

#[derive(Copy, Clone, Debug)]
enum Instr {
	Add { src1: Arg, src2: Arg, dest: Arg },
	Mul { src1: Arg, src2: Arg, dest: Arg },
	Input { dest: Arg },
	Output { src: Arg },
	JumpIfTrue { arg: Arg, target: Arg },
	JumpIfFalse { arg: Arg, target: Arg },
	LessThan { src1: Arg, src2: Arg, dest: Arg },
	Equals { src1: Arg, src2: Arg, dest: Arg },
	Halt,
}
impl Instr {
	fn decode(intcode: &[i32], ip: usize) -> Instr {
		let opcode = intcode[ip];
		let instr = opcode % 100;
		let imm1 = opcode / 100 % 10 != 0;
		let imm2 = opcode / 1000 % 10 != 0;
		let imm3 = opcode / 10000 % 10 != 0;
		match instr {
			1 => Instr::Add {
				src1: Arg::decode(imm1, intcode[ip + 1]),
				src2: Arg::decode(imm2, intcode[ip + 2]),
				dest: Arg::decode(imm3, intcode[ip + 3]),
			},
			2 => Instr::Mul {
				src1: Arg::decode(imm1, intcode[ip + 1]),
				src2: Arg::decode(imm2, intcode[ip + 2]),
				dest: Arg::decode(imm3, intcode[ip + 3]),
			},
			3 => Instr::Input {
				dest: Arg::decode(imm1, intcode[ip + 1]),
			},
			4 => Instr::Output {
				src: Arg::decode(imm1, intcode[ip + 1]),
			},
			5 => Instr::JumpIfTrue {
				arg: Arg::decode(imm1, intcode[ip + 1]),
				target: Arg::decode(imm2, intcode[ip + 2]),
			},
			6 => Instr::JumpIfFalse {
				arg: Arg::decode(imm1, intcode[ip + 1]),
				target: Arg::decode(imm2, intcode[ip + 2]),
			},
			7 => Instr::LessThan {
				src1: Arg::decode(imm1, intcode[ip + 1]),
				src2: Arg::decode(imm2, intcode[ip + 2]),
				dest: Arg::decode(imm3, intcode[ip + 3]),
			},
			8 => Instr::Equals {
				src1: Arg::decode(imm1, intcode[ip + 1]),
				src2: Arg::decode(imm2, intcode[ip + 2]),
				dest: Arg::decode(imm3, intcode[ip + 3]),
			},
			99 => Instr::Halt,
			_ => panic!("Unknown instruction: {}", instr),
		}
	}
	fn advance(&self) -> usize {
		match self {
			Instr::Add { .. } => 4,
			Instr::Mul { .. } => 4,
			Instr::Input { .. } => 2,
			Instr::Output { .. } => 2,
			Instr::JumpIfTrue { .. } => 3,
			Instr::JumpIfFalse { .. } => 3,
			Instr::LessThan { .. } => 4,
			Instr::Equals { .. } => 4,
			Instr::Halt => 1,
		}
	}
}

fn execute(intcode: &[i32], inputs: &[i32]) -> i32 {
	let mut intcode = intcode.to_vec();
	let intcode = &mut intcode[..];

	let mut output = 0;
	let mut input = 0;
	let mut ip = 0;
	loop {
		let instr = Instr::decode(intcode, ip);
		ip += instr.advance();
		match instr {
			Instr::Add { src1, src2, dest } => {
				let value = read(intcode, src1) + read(intcode, src2);
				write(intcode, dest, value);
			},
			Instr::Mul { src1, src2, dest } => {
				let value = read(intcode, src1) * read(intcode, src2);
				write(intcode, dest, value);
			},
			Instr::Input { dest } => {
				write(intcode, dest, inputs[input]);
				input += 1;
			},
			Instr::Output { src } => {
				output = read(intcode, src);
			},
			Instr::JumpIfTrue { arg, target } => {
				if read(intcode, arg) != 0 {
					ip = read(intcode, target) as usize;
				}
			},
			Instr::JumpIfFalse { arg, target } => {
				if read(intcode, arg) == 0 {
					ip = read(intcode, target) as usize;
				}
			},
			Instr::LessThan { src1, src2, dest } => {
				let value = if read(intcode, src1) < read(intcode, src2) { 1 } else { 0 };
				write(intcode, dest, value);
			},
			Instr::Equals { src1, src2, dest } => {
				let value = if read(intcode, src1) == read(intcode, src2) { 1 } else { 0 };
				write(intcode, dest, value);
			},
			Instr::Halt => return output,
		}
	}
}
fn read(intcode: &[i32], arg: Arg) -> i32 {
	match arg {
		Arg::Position(arg) => intcode[arg as usize],
		Arg::Immediate(arg) => arg,
	}
}
fn write(intcode: &mut [i32], arg: Arg, value: i32) {
	match arg {
		Arg::Position(arg) => intcode[arg as usize] = value,
		Arg::Immediate(_) => panic!("Invalid mode for write: {:?}", arg),
	}
}

fn part_one(program: &[i32]) -> i32 {
	let mut outputs = Vec::new();

	for input in vec![0, 1, 2, 3, 4].into_iter().permutations(5) {
		let mut output = 0;
		for inx in input {
			output = execute(program, &[inx, output]);
		}
		outputs.push(output);
	}

	outputs.iter().cloned().max().unwrap()
}

fn part_two(program: &[i32]) -> i32 {
	let mut outputs = Vec::new();

	for input in (5..=9).collect::<Vec<_>>().into_iter().permutations(5) {
		loop {
			let mut output = 0;
			for inx in input.iter().cloned() {
				output = execute(program, &[inx, output]);
			}
			outputs.push(output);
		}
	}

	outputs.iter().cloned().max().unwrap()
}
