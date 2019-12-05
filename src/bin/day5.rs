use std::io;
use std::io::prelude::*;

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let mut program = input.split(",")
		.map(|value| value.parse::<i32>().expect("error parsing opcode"))
		.collect::<Vec<i32>>();

	let result1 = execute(&mut program.clone(), 1);
	print!("Part One\n========\n\nThe computer prints the diagnostic value `{}`.\n\n", result1);

	let result2 = execute(&mut program, 5);
	print!("Part Two\n========\n\nThe computer prints the diagnostic value `{}`.\n\n", result2);
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

fn execute(intcode: &mut [i32], input: i32) -> i32 {
	let mut output = 0;
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
				write(intcode, dest, input);
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

#[test]
fn testing() {
	let mut intcode = [3,0,4,0,99];
	let result = execute(&mut intcode, 42);
	assert_eq!(result, 42);
}

#[test]
fn testingfd() {
	let mut intcode = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
	let result = execute(&mut intcode, 8);
	assert_eq!(result, 1000);
}
