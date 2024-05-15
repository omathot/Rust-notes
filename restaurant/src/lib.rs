//	-	Creating a fron of house module containing hosting and serving submodules.
//	-	Modules can also hold structs, enums, constants, traits and functions
//	-	When navigating code, can refer to this rather than following definition by definition

mod front_of_house;		// NOT LIKE INCLUDE. Every other file will have access to this.

use front_of_house::hosting::add_to_waitlist;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

	// with "use"
	add_to_waitlist();
}

// src/lib.rs compiles to a crate at the root. (same for main.rs)
// structure:
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
