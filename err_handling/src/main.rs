use std::{fs::{self, File}, io::{ErrorKind, Read}, error::Error};

// Debugging
// If successfuly read file, Ok returns file.
// If not succesful, Err returns the error.

// To panic or not to panic?
// prototypes, examples, other TRULY non important things can just panic.
// for bigger projects:
// use expect() and unwrap() as placeholders until you know how to handle different errors.
// use expect() when your code logic ensures you won't have Err, and explain why.
// e.g. Hardcoded strings: let ip = 127.0.0.0 	I made them, they should be valid.
// Returning a Result<> gives the option to handle diff errors, if error is bad bad, can still panic.
// so never really a bad idea to return a Result<> since can always panic after the return

// main() can return values :)
// for now read the return value as "Any kind of error"
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

	// Catching Error in general.
	// if Err not handled, program will crash with panic by default
	// in this case we call panic manually with an error message

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

	// Matching on different errors
	let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Error creating file"),
			},
			other_error => {
				panic!("Problem opening the file!");
			}
		},
    };

	// unwrap is a shortcut method, if open succeeds -> returns file
	// if open fails, panics
	let greeting_file = File::open("hello.txt").unwrap();
	// can use expect() instead of unwrap, here it will print out this message if file not real.
	let greeting_file = File::open("hello.txt").expect("hello.txt should be included");
	// can return easily using ? shortcut
	// function returns: Result<T, E>	where T is type, E is error.
	let mut username_file = File::open("hello.txt")?;	// assigns file or returns err
	let mut username = String::new();
	username_file.read_to_string(&mut username)?;				// assigns username or returns err
	Ok(username);													// returns username if no err
	// can shorten further by chaining ? :
	let mut username = String::new();
	File::open("hello.txt")?.read_to_string(&mut username)?;
	Ok(username);
	// For opening files and putting to a string, use std::fs
	fs::read_to_string("hello.txt");

	// ? can also returns custom structs	e.g. prototype: fn foo() -> my_struct {
	// define: impl From<io::Error>		for my_struct and rust handles the rest 


	
	
	
	
	
	// Cleaner alternative to match using closures, will learn in chapter 13
	// We've seen unwrap_or() before, here we use unwrap_or_else()
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

	Ok(())
}