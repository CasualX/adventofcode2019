use std::io;
use std::io::prelude::*;

/// Reinterpret the string directly as the grid.
/// Compute the stride, width and height from the first line of the input.
fn parse(data: &[u8]) -> Grid<&'_ [u8]> {
	let mut i = 0;
	while data[i] == b'.' || data[i] == b'#' { i += 1; }
	let width = i as i32;
	while data[i] != b'.' && data[i] != b'#' { i += 1; }
	let height = (data.len() / i) as i32;
	let stride = i as i32;
	Grid { data, stride, width, height }
}

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_to_string(&mut input).expect("error reading input");

	let grid = parse(input.as_bytes());

	let (mx, my, mc) = part_one(&grid);

	println!("Part One\n========\n\nFrom `{}, {}` can be seen `{}` asteroids.\n", mx, my, mc);
}

pub struct Grid<T> {
	data: T,
	stride: i32,
	width: i32,
	height: i32,
}
impl<T: AsRef<[u8]>> Grid<T> {
	/// Samples the grid at the given `(x, y)` coordinate.
	pub fn sample(&self, (x, y): (i32, i32)) -> u8 {
		if x < 0 || x >= self.width {
			return b'.';
		}
		if y < 0 || y >= self.height {
			return b'.';
		}
		let index = (y * self.stride + x) as usize;
		self.data.as_ref().get(index).cloned().unwrap_or(b'.')
	}
	/// Calculates if the endpoint is visible from the start point.
	pub fn visible(&self, (startx, starty): (i32, i32), (endx, endy): (i32, i32)) -> bool {
		let (stepx, stepy, steps) = step((endx - startx, endy - starty));
		let mut x = startx;
		let mut y = starty;
		for _ in 0..steps - 1 {
			x += stepx;
			y += stepy;
			if self.sample((x, y)) == b'#' {
				return false;
			}
		}
		true
	}
}

fn gcd(a: i32, b: i32) -> i32 {
	// assert!(a >= 0 && b >= 0);
	if b == 0 { a } else { gcd(b, a % b) }
}

/// Computes the rational step size to reach `(dx, dy)`.
fn step((dx, dy): (i32, i32)) -> (i32, i32, i32) {
	let c = gcd(dx.abs(), dy.abs());
	(dx / c, dy / c, c)
}

fn count_one(grid: &Grid<&[u8]>, (mx, my): (i32, i32)) -> i32 {
	let mut count = 0;
	for ay in 0..grid.height {
		for ax in 0..grid.width {
			if (ax, ay) != (mx, my) && grid.sample((ax, ay)) == b'#' {
				if grid.visible((mx, my), (ax, ay)) {
					count += 1;
				}
			}
		}
	}
	return count;
}

fn part_one(grid: &Grid<&[u8]>) -> (i32, i32, i32) {
	let mut highest = 0;
	let mut x = 0;
	let mut y = 0;
	for my in 0..grid.height {
		for mx in 0..grid.width {
			if grid.sample((mx, my)) == b'#' {
				let count = count_one(grid, (mx, my));
				if count > highest {
					highest = count;
					x = mx;
					y = my;
				}
			}
		}
	}
	return (x, y, highest);
}

#[test]
fn test1fd() {
	let data = &b"\
..#..
.###.
..#..
"[..];
	let grid = Grid { data, stride: 6, width: 5, height: 3 };
	assert_eq!((2, 1, 4), part_one(&grid));
}

#[test]
fn test1a() {
	let data = &b"\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"[..];
	let grid = Grid { data, stride: 11, width: 10, height: 10 };
	assert_eq!((5, 8, 33), part_one(&grid));
}

#[test]
fn test2a() {
	let data = &b"\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
"[..];
	let grid = Grid { data, stride: 11, width: 10, height: 10 };
	assert_eq!((1, 2, 35), part_one(&grid));
}

#[test]
fn test3a() {
	let data = &b"\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
"[..];
	let grid = Grid { data, stride: 11, width: 10, height: 10 };
	assert_eq!((6, 3, 41), part_one(&grid));
}

#[test]
fn test4a() {
	let data = &b"\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"[..];
	let grid = Grid { data, stride: 21, width: 20, height: 20 };
	assert_eq!((11, 13, 210), part_one(&grid));
}

fn part_two(mut grid: &Grid<&mut [u8]>) -> i32 {

}
