
// every variable in rust has a lifetime, most often implied (like types).
// only need explicit lifetime when the lifetimes of references could be related in a few different ways
// main aim of lifetime is to prevent dangling references.
// !! DOES NOT CHANGE LIFETIME, ONLY LETS COMPILER KNOW WHATS UP !!
// let reference;							reference
// let reference_lifetime: &'a i32;		reference with explicit lifetime
// let mut_ref_life: &'a mut i32;

// 'static lifetime lives for the whole duration of the program.
// think about whether it needs to live the wole time before applying it.
// "Most of the time, an error message suggesting the 'static lifetime results from attempting to create
// a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those
// problems, not specifying the 'static lifetime."

use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

	let novel = String::from("I eat food. Shrek is good.");
	let first_sentence = novel.split('.').next().expect("Could not find '.'");
	let i = Excerpt {
		part: first_sentence
	};

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // here we tell compiler return value has same lifetime as arguments
	if x.len() > y.len() {
		x
	}
	else {
		y
	}
}

// struct Excerpt cannot outlive its member part (str slice)
struct Excerpt<'a> {
	part: &'a str,
}

// Generic Type Paramter T
// Here we take any Type <T> that implements Display
fn longest_with_announcement<'a, T>(
	x: &'a str,
	y: &'a str,
	ann: T,
) -> &'a str
where
	T: Display,
{
	println!("Announcement! {}", ann);
	if x > y {
		x
	}
	else {
		y
	}
}





/*
	After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same
	lifetime annotations over and over in particular situations. These situations were predictable and
	followed a few deterministic patterns. The developers programmed these patterns into the compiler’s
	code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit
	annotations.

	This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge
	and be added to the compiler. In the future, even fewer lifetime annotations might be required.
*/