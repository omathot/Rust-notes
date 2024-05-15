fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// to make it mutable ->

fn main() {
    let mut s1 = String::from("hello");

    let len = change(&mut s1);
	println!("{}", s1);
}

fn change(s: &mut String) -> usize {
    s.push_str(", world!");
}

// CANNOT MAKE MULTIPLE REFERENCES TO A MUT REFERENCE
// THIS WONT COMPILE
// made to prevent data races
fn	main() {
	let mut s = String::from("hello");
	
	let r1 = &mut s;
	// {		! If in new scope its okay. Just not in same scope !
	let r2 = &mut s; // second ref here
	// }
	
	println!("{}, {}", r1, r2);
}


// ALSO CANT DO THIS
// immutable references expect their ref not to change. So can't pass a mut one and not together.
fn main() {
	let mut s = String::from("hello");
	
	let r1 = &s; // no problem
	let r2 = &s; // no problem
	// println!("{} and {}", r1, r2);		!! If r1, r2 last used here, r3 is okay. !!
	let r3 = &mut s; // BIG PROBLEM
	// println!("{}", r3);					!! This would work !!
	
	println!("{}, {}, and {}", r1, r2, r3);
}

//-------------------------
//		STRING SLICES
//-------------------------
// from zero to x [..x];
// from x to end [x..];
fn	main() {
	let s = String::from("hello world");

    let hello = &s[0..5]; // same as -> &s[..5];
    let world = &s[6..11];
}

// ! NOTE ! string literals are string slices, no need for slice syntax.
// 			Check last function call
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}












// ---------------------------------------------------------------------------------
// -----------------------------PRE-REFERENCES--------------------------------------
// ---------------------------------------------------------------------------------


// Make s mutable to append to it.
fn main() {
	let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}


// Copies the value of the first variable into the second variable. All Stack operations.
// ! NOTE ! Here x is still valid after being assigned to y.
//  Data type of known size, inexpensive copy, popped out the stack at scope end. No reason to invalidate.
fn	main() {
	let x = 5;
	let y = x;

	println!("x = {x}, y = {y}");
}

// ---------------------------------------------------------------------------------
// String type has:	-pointer to first character
// 					-len of str
// 					-capacity of memory

// Copies the len, capacity and receives same ptr address. !HEAP DATA NOT COPIED! SAME ADDRESS TO SAME STR
// Rust calls this "moving". Spoiler, s1 not valid after being assigned to s2 (read quotes below).
fn	main() {
	let s1 = String::from("Hello");
	let s2 = s1;

	println!("{}", s2);
}

//	- Rust frees all heap allocated memory when variables go out of scope.
//	- So what happens when both s1 and s2 go out of scope? They both point to the same memory address.
//	- double free error? No.
//	- "To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
//		Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens
//		when you try to use s1 after s2 is created; it won’t work:"
//	- "In addition, there’s a design choice that’s implied by this: Rust will never automatically
//		create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive
//		in terms of runtime performance."


//Creates a deep copy using clone(), both s1 and s2 valid.
fn	main() {
	let s1 = String::from("Hello");
	let s2 = s1.clone();

	println!("s1 = {}, s2 = {}", s1, s2);
}




//---------------------------------------------
//			UNDER THE HOOD/CUSTOM
//---------------------------------------------
// data types such as i/u 32 have special anotation "Copy" trait.
// if data type uses "Copy" trait, it is not "moved" but always copied.
// Can't use Copy trait if type or any of its parts use drop()


/*
	Non-exhaustive list of types with Copy trait:
		-	All the integer types, such as u32.
		-	The Boolean type, bool, with values true and false.
		-	All the floating-point types, such as f64.
		-	The character type, char.
		-	Tuples, if they only contain types that also implement Copy. For example,
			(i32, i32) implements Copy, but (i32, String) does not.
*/


// Main note here is takes_ownership(). Note string calls drop() at the end of takes_ownership()
// and not at the end of the scope in which it was created. Can't use s after calling the function.
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------
// Here we give s2 and receive result of function in s3. 
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}