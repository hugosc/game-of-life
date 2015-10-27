extern crate rand;

use std::io;
use rand::distributions::{IndependentSample, Range};

struct GameGrid {
	grid: [[bool; 16]; 16],
	generation: u32
}

impl GameGrid {

	fn set(&mut self, x: usize, y: usize) {
		self.grid[x][y] = true;
	}
	fn clear(&mut self, x: usize, y: usize) {
		self.grid[x][y] = false;
	}
	fn advance_generation(&mut self) -> bool {

		fn count_neighbours(grid: &[[bool; 16]; 16], x: usize, y: usize)
		-> usize {

			let mut neighbour_count = 0;

			neighbour_count += if x < 15 && y < 15 { grid[x+1][y+1] as usize } else { 0 };
			neighbour_count += if x < 15 && y > 0  { grid[x+1][y-1] as usize } else { 0 };
			neighbour_count += if x > 0  && y < 15 { grid[x-1][y+1] as usize } else { 0 };
			neighbour_count += if x > 0  && y > 0  { grid[x-1][y-1] as usize } else { 0 };
			neighbour_count += if x < 15           { grid[x+1][y]   as usize } else { 0 };
			neighbour_count += if y < 15           { grid[x][y+1]   as usize } else { 0 };
			neighbour_count += if x > 0  		   { grid[x-1][y]   as usize } else { 0 };
			neighbour_count += if y > 0  		   { grid[x][y-1]   as usize } else { 0 };

			neighbour_count
		}

		let grid = self.grid;

		let mut changed = false;

		for x in 0..15 {
			for y in 0..15 {

				match (self.grid[x][y], count_neighbours(&grid, x, y)) {
					(true, 0) | (true, 1) => self.grid[x][y] = false,
					(true, 2) | (true, 3) => self.grid[x][y] = true,
					(true, _)             => self.grid[x][y] = false,
					(false, 3)            => self.grid[x][y] = true,
					(false, _)            => self.grid[x][y] = false
				}

				changed = changed || (self.grid[x][y] != grid[x][y]);
			}
		}

		self.generation += 1;	

		changed
	}

	fn print(&self) {
		println!("  -------------------------------");
		for i in 0..15 {
			print!(" | ");
			for j in 0..15 {
				let block = if self.grid[i][j] {'O'} else { '.' };
				print!("{} ", block);
			}
			print!("|\n");
		}
		println!("  -------------------------------");
	}
}

fn random_grid(density: f32) -> GameGrid {

	let between = Range::new(0 as f32, 1 as f32);
    let mut rng = rand::thread_rng();

	let mut rg = GameGrid { 
		grid: [[false; 16]; 16],
		generation: 0
	};
	for x in 0..15 {
		for y in 0..15 {

			let coin_flip = (density - between.ind_sample(&mut rng)) > 0 as f32;
			if coin_flip { rg.set(x,y) ;} else { rg.clear(x,y); }
		}
	}
	rg
}

fn main() {

	let mut input = String::new();

	let mut game_grid = random_grid(0.0);

	loop {

		io::stdin().read_line(&mut input)
        	.ok()
        	.expect("failed to read line");

	    let input = input.trim();

	    if input == "quit".to_string() || input == "q".to_string() {
	 		break;
	 	}

	 	game_grid.print();

	 	if !game_grid.advance_generation() { break; }
	}
}