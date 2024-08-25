use std::{borrow::Borrow, cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
	value: i32,
	parent: RefCell<Weak<Node>>, // child should not own parent, if child gets dropped parent should exist.
	children: RefCell<Vec<Rc<Node>>>, // parent should own child, if parent gets dropped child should drop.
}

fn main() {
    let leaf = Rc::new(Node {
		value: 3,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![]),
	});
	println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade()); // no parent yet, returns None
	println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

	{
		let branch = Rc::new(Node {
			value: 5,
			parent: RefCell::new(Weak::new()),
			children: RefCell::new(vec![Rc::clone(&leaf)]),
		});
		*leaf.parent.borrow_mut() = Rc::downgrade(&branch); // assign parent to leaf
		println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade()); // parent assigned, returns branch.
		println!("branch strong = {}, branch weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // 1strong 1weak, itself and its child
		println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 2 strong, itself and parent
	}
	println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade()); // no parent, branch dropped out of scope
	println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
