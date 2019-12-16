use std::io;
use std::io::prelude::*;

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let program = input.split(",")
		.map(|value| value.parse::<i64>().expect("error parsing opcode"))
		.collect::<Vec<i64>>();

	let result1 = execute(&mut program.clone(), 1);
	print!("Part One\n========\n\nThe computer produces the BOOST keycode `{}`.\n\n", result1);

	let result2 = execute(&mut program.clone(), 2);
	print!("Part Two\n========\n\nThe coordinates of the distress signal is `{}`.\n\n", result2);
}


#[derive(Copy, Clone, Debug)]
enum Arg {
	Position(i64),
	Immediate(i64),
	Relative(i64),
}
impl Arg {
	fn decode(mode: u8, arg: i64) -> Arg {
		match mode {
			0 => Arg::Position(arg),
			1 => Arg::Immediate(arg),
			2 => Arg::Relative(arg),
			_ => panic!("invalid mode: {} with arg: {}", mode, arg),
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
	AdjustRelBase { value: Arg },
	Halt,
}
impl Instr {
	fn decode(intcode: &[i64], ip: usize) -> Instr {
		let opcode = intcode[ip];
		let instr = opcode % 100;
		let mode1 = (opcode / 100 % 10) as u8;
		let mode2 = (opcode / 1000 % 10) as u8;
		let mode3 = (opcode / 10000 % 10) as u8;
		match instr {
			1 => Instr::Add {
				src1: Arg::decode(mode1, intcode[ip + 1]),
				src2: Arg::decode(mode2, intcode[ip + 2]),
				dest: Arg::decode(mode3, intcode[ip + 3]),
			},
			2 => Instr::Mul {
				src1: Arg::decode(mode1, intcode[ip + 1]),
				src2: Arg::decode(mode2, intcode[ip + 2]),
				dest: Arg::decode(mode3, intcode[ip + 3]),
			},
			3 => Instr::Input {
				dest: Arg::decode(mode1, intcode[ip + 1]),
			},
			4 => Instr::Output {
				src: Arg::decode(mode1, intcode[ip + 1]),
			},
			5 => Instr::JumpIfTrue {
				arg: Arg::decode(mode1, intcode[ip + 1]),
				target: Arg::decode(mode2, intcode[ip + 2]),
			},
			6 => Instr::JumpIfFalse {
				arg: Arg::decode(mode1, intcode[ip + 1]),
				target: Arg::decode(mode2, intcode[ip + 2]),
			},
			7 => Instr::LessThan {
				src1: Arg::decode(mode1, intcode[ip + 1]),
				src2: Arg::decode(mode2, intcode[ip + 2]),
				dest: Arg::decode(mode3, intcode[ip + 3]),
			},
			8 => Instr::Equals {
				src1: Arg::decode(mode1, intcode[ip + 1]),
				src2: Arg::decode(mode2, intcode[ip + 2]),
				dest: Arg::decode(mode3, intcode[ip + 3]),
			},
			9 => Instr::AdjustRelBase {
				value: Arg::decode(mode1, intcode[ip + 1]),
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
			Instr::AdjustRelBase { .. } => 2,
			Instr::Halt => 1,
		}
	}
}

fn execute(intcode: &mut Vec<i64>, input: i64) -> i64 {
	let mut output = 0;
	let mut ip = 0;
	let mut rel_base = 0;
	loop {
		let instr = Instr::decode(intcode, ip);
		ip += instr.advance();
		match instr {
			Instr::Add { src1, src2, dest } => {
				let value = read(intcode, src1, rel_base) + read(intcode, src2, rel_base);
				write(intcode, dest, rel_base, value);
			},
			Instr::Mul { src1, src2, dest } => {
				let value = read(intcode, src1, rel_base) * read(intcode, src2, rel_base);
				write(intcode, dest, rel_base, value);
			},
			Instr::Input { dest } => {
				write(intcode, dest, rel_base, input);
			},
			Instr::Output { src } => {
				output = read(intcode, src, rel_base);
			},
			Instr::JumpIfTrue { arg, target } => {
				if read(intcode, arg, rel_base) != 0 {
					ip = read(intcode, target, rel_base) as usize;
				}
			},
			Instr::JumpIfFalse { arg, target } => {
				if read(intcode, arg, rel_base) == 0 {
					ip = read(intcode, target, rel_base) as usize;
				}
			},
			Instr::LessThan { src1, src2, dest } => {
				let value = if read(intcode, src1, rel_base) < read(intcode, src2, rel_base) { 1 } else { 0 };
				write(intcode, dest, rel_base, value);
			},
			Instr::Equals { src1, src2, dest } => {
				let value = if read(intcode, src1, rel_base) == read(intcode, src2, rel_base) { 1 } else { 0 };
				write(intcode, dest, rel_base, value);
			},
			Instr::AdjustRelBase { value } => {
				rel_base += read(intcode, value, rel_base);
			},
			Instr::Halt => return output,
		}
	}
}
fn read(intcode: &[i64], arg: Arg, rel_base: i64) -> i64 {
	let address = match arg {
		Arg::Position(arg) => arg,
		Arg::Immediate(arg) => return arg,
		Arg::Relative(arg) => rel_base + arg,
	};
	if address < 0 {
		panic!("invalid read address: {}", address);
	}
	intcode.get(address as usize).cloned().unwrap_or(0)
}
fn write(intcode: &mut Vec<i64>, arg: Arg, rel_base: i64, value: i64) {
	let address = match arg {
		Arg::Position(arg) => arg,
		Arg::Immediate(_) => panic!("Invalid mode for write: {:?}", arg),
		Arg::Relative(arg) => rel_base + arg,
	};
	if address < 0 {
		panic!("invalid write address: {}", address);
	}
	if address as usize >= intcode.len() {
		intcode.resize_with(address as usize + 1, Default::default);
	}
	intcode[address as usize] = value;
}

#[test]
fn test1a() {
	let mut program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
	let _output = execute(&mut program, 0);
}
#[test]
fn test2a() {
	let mut program = vec![1102,34915192,34915192,7,4,7,99,0];
	let output = execute(&mut program, 0);
	assert_eq!(output.to_string().len(), 16); // Output 16 digit number
}
#[test]
fn test3a() {
	let mut program = vec![104,1125899906842624,99];
	let output = execute(&mut program, 0);
	assert_eq!(output, 1125899906842624); // Output the large number in the middle
}
