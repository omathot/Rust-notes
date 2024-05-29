mod phonebook;

use std::process::exit;
use::std::{io::{self, Write}, env};
use crate::phonebook::{Phonebook, Inputs};

fn main() {
    let args_n = env::args().count();
	if args_n != 1 {
		println!("Usage: cargo run");
	}

	let mut book = init_book();
	loop {
		print!("Usage: ADD, SEARCH, EXIT >> ");
		let _ = io::stdout().flush();
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Invalid usage!");

		let input = input.trim();
		match Inputs::from_string(&input) {
			Inputs::ADD => {
				book.add_contact();
			}
			Inputs::SEARCH => { 
				book.search();
			}
			Inputs::EXIT => {
				exit(0);
			}
			_ => println!("Invalid input"),
		}
	}
}

fn init_book() -> Phonebook {
	let book = Phonebook {
		max_contacts: 8,
		num_contacts: 0,
		contacts: Vec::new(),
		next_replace: 0,
	};
	book
}