use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

//	-	VECTORS, STRINGS & HASHMAPS
fn main() {
	//-------------------------------------
	//				VECTORS
	//-------------------------------------
	println!("-----------------\n-----VECTORS-----\n-----------------");

    // Create new empty Vector
	let _v: Vec<i32> = Vec::new();

	// Create vector with values
	let mut v = vec![1, 3, 4];

	// Add elements to Vector
	v.push(5);
	v.push(6);
	v.push(7);

	// Reading elements of Vector
	// Use direct reference if you want program to crash when accessing out of range.
	// Use get() if occasional out of range is normal, e.g. User input. can input wrong number.
	println!("\nReading Vector");
	let _first = &v[0];						// this returns i32, will panic if out of range.
	println!("Value at v[0] = {_first}");
	let out_of_range = v.get(11);	// this returns Options<T>, if out of range returns None
	match out_of_range {
		Some(out_of_range) => println!("v[1] = {}", out_of_range),
		None => println!("No value at v[11]!"),
	}

	// Iterating through a Vector
	println!("\nIterating through Vector");
	for i in &mut v {
		println!("{i}");
		*i += 50;
	}

	// Vectors can only contain one type. To get around that we can use Enums.
	// Enums can store different types and group them under their Enum type.
	println!("\nVector with Enums with different types");
	let diff_type_v = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Float(3.2),
		SpreadsheetCell::Text(String::from("blue")),
	];
	for i in &diff_type_v {
		println!("{:?}", i);
	}

	//-------------------------------------
	//				STRINGS
	//-------------------------------------
	println!("\n-----------------\n-----STRINGS-----\n-----------------");

	// Create string with new()
	let mut _s = String::new();

	// Create String with to_string() / from()
	let hello = "Hello";
	let s = hello.to_string();
	let s = "Hello".to_string();
	let mut s = String::from("Hello");

	// Push to string
	println!("\nPushing character ',' and str ' world!' to 'Hello'");
	s.push(',');
	s.push_str(" world!");
	println!("String 'Hello' after push = {s}");

	// Adding strings
	// add operator defined like this :
	// fn add(self, s: &str) -> String;
	// so give reference to String for 2nd argument.
	// 2nd argument still valid after operation, only passed a reference to it.
	println!("\nUsing String + &str to add 2 strings");
	let hello = String::from("Hello");
	let world = String::from(", world!");
	let full_string = hello + &world;		// Hello is moved here, can't access after full_string in scope.
	println!("full_string = {full_string}");

	// format takes references so it doesn't take ownership (that's good)
	println!("\nUsing format!() to make String");
	let s1 = String::from("Tic");
	let s2 = String::from("Tac");
	let s3 = String::from("Toe");
	let full_string = format!("{s1}-{s2}-{s3}"); // prints Tic-Tac-Toe
	println!("full_string = {}", full_string);

	// Indexing into Strings
	// no indexing is safe, gives this example: let hello = String::from("Здравствуйте");
	// 12 characters, but 24 bites. each char is 2 bites for Cyrillic.
	// If we received this string expecting 12 bites, we'd be indexing into it wrong.
	// if need more details just go read this https://doc.rust-lang.org/book/ch08-02-strings.html
	println!("\nIndexing into strings: You can't do str[n]!\nYou can use slices instead:");
	println!("full_string = {}", full_string);
	let part = &full_string[0..4];
	println!("Slice of full string [0..4] = {}", part);
	println!("\nYou can iterate through a string by specifiying what type you're expecting");
	println!("Iterating through string 'hello':\n	-retrieving characters");
	// Here we specify "Hello".chars() to retrive the character.
	for c in "Hello".chars() {
		println!("{c}");
	}
	println!("	-retrieving bytes");
	// Here we specify "Hello".bytes() to retrive the raw byte data.
	for b in "Hello".bytes() {
		println!("{b}");
	}
	// Use contains() and replace() methods on Strings to look for something, or to replace a part of a str.


	//-------------------------------------
	//				HASH MAPS
	//-------------------------------------
	// Default Hash algorithm is SipHash (not fastest, but safe), can use others found on crates.io
	println!("\n-------------------\n-----HASH MAPS-----\n-------------------");
	// HashMap<Key, Value>
	// Al keys must have same type, all Values must have same type.
	println!("Creating HashMap and putting scores inside");
	// Create new HashMap using new + insert.
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 20);
	
	// Accessing values
	// get() returns Option<&v> (in this case &i32)
	// copied() converts it to Option<v> (i32)
	// unwrap() returns the Some value, unwrap_or() adds or statement if None (in this case set score to 0)
	// unwrap_or_else() more appropriate if passing result of function call.
	println!("Blue score = {}\nYellow score = {}", scores.get("Blue").copied().unwrap_or(0), scores.get("Yellow").copied().unwrap_or(0));
	
	// let team_name = String::from("Blue");
	// let score = scores.get(&team_name).copied();
	
	// iterating through HashMap
	println!("\nIterating through HashMap");
	{
		for (key, value) in &scores {
			println!("{key}: {value}");
		}
	}
	
	// Updating HashMap
	// Overwriting
	println!("\nUpdating HashMap:\nOverwriting value of Blue");
	scores.insert(String::from("Blue"), 99);
	println!("Blue now = {}", scores.get("Blue").copied().unwrap());

	// Add Key and Value only if there is no matching Key using entry()
	println!("\nUsing entry() to attempt to insert value for keys Blue and Purple");
	scores.entry(String::from("Blue")).or_insert(98);
	scores.entry(String::from("Purple")).or_insert(30);

	for (key, value) in &scores {
		println!("{key}{value}");
	}

	// Updating based on old value
	println!("\nUpdating based on old value");
	let text = "Hello world wonderful world";
	let mut map = HashMap::new();
	println!("Parsing string '{text}'");
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);	// returns &mut i32, no need to make count mut
		*count += 1;
	} // count goes out of scope here, borrow checker happy
	println!("{:?}", map);

	
}
