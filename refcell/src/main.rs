// this is what refcell lets you do under the hood.
	// let x = 5;
	// let y = &mut x; here rust says no, can't mutate non mut var. RefCell will let you do that.

// What are they good for?
// Mock Objects
// Mock objects are specific types of test doubles that record what happens during a test so you
// can assert that the correct actions took place.

/*
Here’s the scenario we’ll test: we’ll create a library that tracks a value against a maximum
value and sends messages based on how close to the maximum value the current value is.
This library could be used to keep track of a user’s quota for the number of API calls they’re
allowed to make, for example. 
*/

use std::{cell::RefCell, rc::Rc};

pub trait Messenger {
	fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
	messenger: &'a T,
	value: usize,
	max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
	pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
		LimitTracker {
			messenger,
			value: 0,
			max,
		}
	}

	pub fn set_value(&mut self, value: usize) {
		self.value = value;

		let percentage_of_max = self.value as f64 / self.max as f64;
		if percentage_of_max >= 1.0 {
			self.messenger.send("You are over your quota!");
		} else if percentage_of_max >= 0.9 {
			self.messenger.send("Urgent warning: You are over 90% of your quota!");
		} else if percentage_of_max >= 0.75 {
			self.messenger.send("Warning: You are over 75% of your quota!");
		}
	}
}

#[derive(Debug)]
enum List {
	Cons(Rc<RefCell<i32>>, Rc<List>),
	Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));

	let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

	let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
	let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
	*value.borrow_mut() = 10;

	println!("a after = {a:?}");
	println!("b after = {b:?}");
	println!("c after = {c:?}");
}

#[cfg(test)]
mod tests {
	use std::cell::RefCell;
	use super::*;

	struct MockMessenger {
		sent_messages: RefCell<Vec<String>>,
	}
	impl MockMessenger {
		fn new() -> MockMessenger {
			MockMessenger {
				sent_messages: RefCell::new(vec![]),
			}
		}
	}
	impl Messenger for MockMessenger {
		fn send(&self, msg: &str) {
			self.sent_messages.borrow_mut().push(String::from(msg));
		}
	}

	#[test]
	fn it_sends_an_over_75_percent_warning_message() {
		let mock_messenger = MockMessenger::new();
		let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

		limit_tracker.set_value(80);
		assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
	}
}