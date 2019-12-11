use std::io;
use std::io::prelude::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

fn parse(s: &str) -> Vec<u8> {
	s.bytes().collect()
}

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");
	let pixels = parse(&input);

	let num_layers = pixels.len() / SIZE;
	let mut least_zeroes = 9999999;
	let mut checksum = 0;
	for i in 0..num_layers {
		let layer = &pixels[i * SIZE..(i + 1) * SIZE];
		let zeroes = layer.iter().filter(|&&x| x == b'0').count();
		if zeroes < least_zeroes {
			least_zeroes = zeroes;
			let ones = layer.iter().filter(|&&x| x == b'1').count();
			let twos = layer.iter().filter(|&&x| x == b'2').count();
			checksum = ones * twos;
		}
	}

	println!("Part One\n========\n\nThe number is `{}`.", checksum);

	println!("Part Two\n========\n\n```\n");
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			for i in 0..num_layers {
				let layer = &pixels[i * SIZE..(i + 1) * SIZE];
				let pixel = layer[y * WIDTH + x];
				if pixel != b'2' {
					print!("{}", if pixel == b'1' { '#' } else { ' ' });
					break;
				}
			}
		}
		println!();
	}
	println!("```\n");
}
