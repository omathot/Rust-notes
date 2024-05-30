use std::{env, process};
use minigrep::Config;

/*
Separation of responsability:
	- Put all program logic in lib.rs
	- Main only responsible for: 1. parsing, 2. setting up configs, 3. run lib, 4. handling error returns.
*/

fn main() {
    let args: Vec<String> = env::args().collect();
	let config = Config::build(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments : {}", err);
		process::exit(1);
	});
	// using if let, run returns () on success so we only check for Err return.
	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}

