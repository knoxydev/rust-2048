#![allow(warnings)]

//PACKAGES
use rand::Rng;

// MODULES
mod keyboard;
pub use crate::keyboard::keyboard_md;


pub struct Main
{
	board: [[i64; 4]; 4],
}

impl Main
{
	fn print(&self)
	{
		println!("\n");
		for i in self.board { println!(" {:?} {:?} {:?} {:?}", i[0], i[1], i[2], i[3]); }
		println!("\n");
	}


	fn first_gen(&mut self)
	{
		let random = || -> i64
		{
			let mut rng = rand::thread_rng();
	    let num: i64 = rng.gen_range(0..=3);
    	return num;
		};

		{
	    self.board[random() as usize][random() as usize] = 2;
	    self.board[random() as usize][random() as usize] = 2;
	  }
	}


	fn up(&self)
	{
		println!("up");
	}


	fn left(&self)
	{
		println!("left");
	}


	fn down(&self)
	{
		println!("down");
	}


	fn right(&self)
	{
		println!("right");
	}
}


fn main()
{
	let mut main = Main {
 		board: [
 			[0, 0, 0, 0],
 			[0, 0, 0, 0],
 			[0, 0, 0, 0],
 			[0, 0, 0, 0],
 		],
 	};

 	main.first_gen();
 	main.print();

 	keyboard_md::start(&main);
}
