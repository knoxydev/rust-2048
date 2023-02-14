#![allow(warnings)]

// MODULES
mod keyboard;
pub use crate::keyboard::keyboard_md;


pub struct Main
{
	board: [[i64; 4]; 4],
}

impl Main
{
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
	let main = Main {
 		board: [
 			[0, 0, 0, 0],
 			[0, 0, 0, 0],
 			[0, 0, 0, 0],
 			[0, 0, 0, 0],
 		],
 	};

 	keyboard_md::start(&main);

 	//println!("{:?}", main.board);
}
