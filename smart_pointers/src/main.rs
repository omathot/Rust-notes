use std::{ops::Deref, rc::Rc};

// need box<T> to allocate on heap, so rust compiler knows size of memory address.
enum List {
	Cons(i32, Box<List>),
	Nil,
}

// use Rc<T> instead of Box<T>, so it can have multiple references to it. (graph image example)
enum RefCountedList {
	CountedCons(i32, Rc<RefCountedList>),
	Nul,
}

struct MyBox<T>(T); // struct containing a tuple of type <T>
impl<T> MyBox<T> {
	fn new(x:T) -> MyBox<T> {
		MyBox(x)
	}
}
impl<T> Deref for MyBox<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

struct CustomSmartPointer {
	data: String,
}
impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data {} !", self.data);
	}
}

use List::{Cons, Nil};
use RefCountedList::{CountedCons, Nul};


// using deref coercion to turn type<T> into String and dereference into &str.
// number of times deref needs to be called is resolved at compile time, so no runtime performance cost.
fn hello(name: &str) {
	println!("Hello {name}");
}

fn main() {
	// just assigning i32 to heap, printing its value and address
	let b = Box::new(5);
	println!("b = {b}, {:p}", &b);

	let p = 5;
	let p_mybox = MyBox(p);
	assert_eq!(5, *p_mybox); // here we use Deref trait we implemented on MyBox

	let string_test = MyBox::new(String::from("Rust"));
	hello(&string_test);

	let s1 = CustomSmartPointer {
		data: String::from("My stuff"),
	};
	// can also drop early using drop()
	println!("s1 created");
	drop(s1);
	println!("s1 dropped early");
	
	let s2 = CustomSmartPointer {
		data: String::from("Other stuff"),
	};
	println!("s2 created");
	// create list[0] with value (1, new_list), list [1] with value{2, new_list}, list[2] (3, Nil) <- end of list;
	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

	// create RefCountedList, will be valid as long as there are references to it. itself is a ref to it too.
	let count_a = Rc::new(CountedCons(1, Rc::new(CountedCons(2, Rc::new(Nul)))));
	println!("RefCount after creating count_a: {}", Rc::strong_count(&count_a));
	let b_ref_a = CountedCons(3, Rc::clone(&count_a));
	println!("RefCount after creating b_ref_a: {}", Rc::strong_count(&count_a));
	{
		let c_ref_a = CountedCons(4, Rc::clone(&count_a));
		println!("RefCount after creating c_ref_a: {}", Rc::strong_count(&count_a));
	}
	println!("RefCount after c_ref_a goes out of scope: {}", Rc::strong_count(&count_a));
}

/*
Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
 */



// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// ---------------------------------ADVANCED---------------------------------
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
// Rc<> use case example for nicer inits of nested structs

/*
struct Test {
    test: Option<Rc<Test>>
}

impl Test {
    fn new() -> Rc<Self> {
        Rc::new(Test { test: None })
    }

    fn with_child(child: Rc<Test>) -> Rc<Self> {
        Rc::new(Test { test: Some(child) })
    }

    fn chain(depth: usize) -> Rc<Self> {
        if depth == 0 {
            Test::new()
        } else {
            Test::with_child(Test::chain(depth - 1))
        }
    }
}

fn main() {
    // Create a chain of 3 Test instances
    let test = Test::chain(3);
    
    // Or build it step by step
    let test = Test::with_child(
        Test::with_child(
            Test::new()
        )
    );
} */