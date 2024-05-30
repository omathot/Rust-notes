// !! MAKE SURE TO READ TO THE BOTTOM - GOOD PRACTICE INFO !!

/*
	Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
	The bodies of test functions typically perform these three actions:

	1. Set up any needed data or state.
	2. Run the code you want to test.
	3. Assert the results are what you expect.

	most often in library
	declare a test function with #[test] on the line before fn x()
	run tests with:  cargo test
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// assert!() evaluates bool						--> does not print the values
// assert_eq!() shortcut for assert!(x == y)	--> prints the values
// assert_ne!() shortcut for assert!(x != y)	--> prints the values
// custom types need to implement Debug and PartialEq
pub fn add(left: usize, right: usize) -> usize {
	left + right
}
pub fn add_two(x: usize) -> usize {
	x + 2
}

// purposeful bug
pub fn greetings(name: &str) -> String {
	format!("Hello!")
}

// can test for intended panic with should_panic
struct Guess {
	n: i32,
}
// purposeful switch of panic strings
impl Guess {
	pub fn new(n: i32) -> Guess {
		if n < 1 {
			panic!("Value should be less than or equal to 100, got {}", n);
		}
		else if n > 100 {
			panic!("Value should be greater than 0, got {}", n);
		}
		Guess{n}
	}
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn larger_can_hold_smaller() {
		let larger = Rectangle {
			width: 10,
			height: 10,
		};
		let smaller = Rectangle {
			width: 5,
			height: 5,
		};
		assert!(larger.can_hold(&smaller));
	}

	#[test]
	fn smaller_cannot_hold_larger() {
		let larger = Rectangle {
			width: 10,
			height: 10,
		};
		let smaller = Rectangle {
			width: 5,
			height: 5,
		};
		assert!(!smaller.can_hold(&larger));
	}

	#[test]
	fn it_adds_two() {
		let x = 2;
		assert_eq!(4, add_two(x));
	}

	#[test]
	fn greetings_contains_name() {
		let name = greetings("Carolel");
		assert!(
			name.contains("Carolel"),
			"Greeting did not contain name, value was '{}'",
			name
		);
	}

	#[test]
	#[should_panic(expected = "less than or equal to 100")]
	fn greater_than_100() {
		Guess::new(101);
	}
}

// tests run concurrently by default, can control with cargo test -- --test-threads=1
// example: tests that write to the same file concurrently will mess up the expected output.
// can show output of successful tests with cargo test -- --show-output
// can run specific test with cargo test --<function name>
// can run selection of tests with a common word in their fn name, cargo test --<common word>
// can ignore specific tests that take a long time with #[ignore], same use as #[should_panic]
// can run ignored tests with cargo test -- --ignored



// rust community thinks of 2 main different types of tests
// 	- Unit tests
//  	- small and focused, testing one module at a time and private interfaces
//  - Integration tests
//  	- Uses code through API as anyone would use your library.
//  	- Uses only public interfaces, and can test multiple modules at once.

/*
Unit tests:
	- Put tests module in each src/ files to test the respective code.
	- Declare it with #[cfg(test)]	--> compiler doesn't take those into account when cmd cargo build
	- cfg boils down to "configuration", does not compile config'd items unless given the config (in this case test)

Integration tests:
	- External to library, only use public API, intended to test multiple parts of your code working together.
	- Goes into dedicated tests/ directory at the root, next to src/
	- When project grows seems to get more complex, but we'll see when we get there
*/
