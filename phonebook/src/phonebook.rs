mod contact;

use std::io::{self, Write};
use crate::phonebook::contact::Contact;


#[derive(Debug)]
pub struct Phonebook {
	pub max_contacts: u8,
	pub contacts: Vec<Contact>,
	pub num_contacts: i8,
	pub next_replace: usize,
}

impl  Phonebook{
	pub fn add_contact(&mut self) {
		let first_name = loop {
			match prompt("First name >> ") {
				Ok(name) => break name,
				Err(err) => println!("{}", err),
			}
		};
		let last_name = loop {
			match prompt("Last name >> ") {
				Ok(name) => break name,
				Err(err) => println!("{}", err),
			}
		};
		let nickname = loop {
			match prompt("Nickname >> ") {
				Ok(nick) => break nick,
				Err(err) => println!("{}", err),
			}
		};
		let phone_nbr = loop {
			match prompt("Phone number >> ") {
				Ok(nbr) => {
					if nbr.chars().any(|c| c.is_alphabetic()) {
						println!("Phone number can only contain digits.");
					}
					else {
						break nbr
					}
				},
				Err(err) => println!("{}", err),
			}
		};
		let worst_nightmare = loop {
			match prompt("Worst nightmare >> ") {
				Ok(name) => break name,
				Err(err) => println!("{}", err),
			}
		};
		let new_contact = Contact {
			first_name,
			last_name,
			nickname,
			phone_nber: phone_nbr,
			worst_nightmare,
		};

		if self.contacts.len() < 8 {				// self.max_contacts::<usize>() u8 instead of usize
			self.contacts.push(new_contact);
		}
		else {
			self.contacts[self.next_replace] = new_contact;
			self.next_replace = (self.next_replace + 1) % 8;
		}
	}

	pub fn search(&self) {
		self.print_contacts();
		loop {
			let input = match prompt("Selext index >> ") {
				Ok(value) => value,
				Err(_) => {
					println!("Must enter a value");
					continue ;
				}
			};

			match input.trim().parse::<usize>() {
				Ok(index) => {
					if index < self.contacts.len() {
						println!("\n{:<10} | {:<10} | {:<10} | {:<10} | {:<10}",
						"First name", "Last name", "Nickname", "Phone", "Nightmare");
						println!("{:<10} | {:<10} | {:<10} | {:<10} | {:<10}",
						self.contacts[index].first_name, self.contacts[index].last_name,
						self.contacts[index].nickname, self.contacts[index].phone_nber,
						self.contacts[index].worst_nightmare);
						break ;
					}
					else {
						println!("Invalid index, out of range");
					}
				}
				Err(_) => {
					println!("Enter a digit to navigate using index");
				}
			}
		}
	}
	pub fn print_contacts(&self) {
		println!("\n{:<10} | {:<10} | {:<10}", "Index", "First name", "Last name");
		for (index, contact) in self.contacts.iter().enumerate() {
			println!("{:<10} | {:<10} | {:<10}", index, contact.first_name, contact.last_name);
		}
	}
}

pub enum Inputs {
	ADD,
	SEARCH,
	EXIT,
	UNKNOWN(String),
}

impl Inputs {
	pub fn from_string(input: &str) -> Inputs {
		match input.to_lowercase().trim() {
			"add" => Inputs::ADD,
			"search" => Inputs::SEARCH,
			"exit" => Inputs::EXIT,
			other => Inputs::UNKNOWN(other.to_string()),
		}
	}
}

// string as error
// anyhow package
// cargo clippy
// &'static str -> use static ref to string
fn prompt(message: &str) -> Result<String, &'static str> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read line")?;
    
	let trimmed = input.trim();
	if trimmed.is_empty() {
			Err("Input can't be empty")
	}
	else {
		Ok(trimmed.to_string())
	}
}