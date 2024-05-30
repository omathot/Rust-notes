use std::{error::Error, fs, env};

pub struct Config {
	pub file_name: String,
	pub query: String,
	pub ignore_case: bool,
}
impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments");
		}
		let query = args[1].clone();
		let file_name = args[2].clone();
		let ignore_case = env::var("IGNORE_CASE").is_ok();	// is_ok() only checks if var is set, don't care for value of variable

		Ok(Config {
			query,
			file_name,
			ignore_case,
		})
	}
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.file_name)?;
	
	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &content)
	}
	else {
		search(&config.query, &content)
	};
	for line in results {
		println!("{}", line);
	}

	Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	
	for line in content.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	let query = query.to_lowercase();

	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "Shrek";
		let content = "Sun is bright.\n\
		Sky is blue.\n\
		Doors are wood.\n\
		I eat food. Shrek is good.\n\
		Socks smell good.\n\
		Coffers are full of Shrek.\n\
		";

		assert_eq!(vec!["I eat food. Shrek is good.", "Coffers are full of Shrek."], search(query, content));
	}

	#[test]
	fn case_sensitive() {
		let query = "shrek";
		let content = "\
Shrek is powerful
shrek miniscule";

		assert_eq!(vec!["shrek miniscule"], search(query, content));
	}
	#[test]
	fn case_insensitive() {
		let query = "shrek";
		let content = "\
Shrek is powerful
shrek is love";

		assert_eq!(vec!["Shrek is powerful", "shrek is love"], search_case_insensitive(query, content));
	}
}