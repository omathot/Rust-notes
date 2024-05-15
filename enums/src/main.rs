//	-	Enums are usually used when can enumerate all possible options
//	-	For example, all possible IP types we can run into.
//	-	Another example: structure shape (circle, rectangle, square, triangle...)
//	-	each variant can have different types and quantity of data.
//	-	V4(u8, u8, u8, u8) works like V4(String) in this case.
//	-	Can also put a custom struct or enum as data type
//	-	! Can use impl for enums too !
#[derive(Debug)]
enum IpAddr {
	V4(u8, u8, u8, u8),
	// V4(String),
	V6(String),
}

impl IpAddr {
	fn call(&self) {
		dbg!(self);
	}
}

//	-	"IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type."
fn main() {
	let home = IpAddr::V4(127, 0, 0, 1);
	let	loopback = IpAddr::V6(String::from("::1"));
	home.call();
	loopback.call();
	
	//-------------
	// ! If we were to use structs instead (worse in this scenario) !
	//-------------
	// let	home = IpAddr {
	// 	kind: IpAddrKind::V4,
	// 	address: String::from("120.0.0.1"),
	// };
    // let	loopback = IpAddr {
	// 	kind: IpAddrKind::V6,
	// 	address: String::from("::1"),
	// };

}


//----------------------------------------
//				OPTION ENUM
//----------------------------------------
//	-	In std library. 
//	-	Used to check if value is something or nothing.
//	-	First field of empty array is nothing.
//	-	There is no NULL in rust.

// This is how Options Enum is defined, it is in the prelude, meaning can access by default
//	enum Option<T> {
//		None,
//		Some(T),
//	}

//	-	! Here, can't add some_number and x, they are different types. !
//	-	When putting type <T> in Some, need to convert Option<T> to <T> before doing <T> operations.
fn main() {
	let mut some_number = Some(5);
	let	some_char = Some('e');
	let	unknown_number: Option<i32> = None;		// Setting to None (NULL), need to define what type.

	some_number.is_some();						// check if some
	some_number.is_some_and(|x| x > 1);	// Check if some and if value inside matches given predicate
	some_number.as_ref();						// Converts from &Option<T> to Option<&T>.
	some_number.as_mut();						// Converts from &mut Option<T> to Option<&mut T>.
	some_number.insert(12);						// Inserts value 12. Not sure how this behaves yet.

	
	let	x = 3;	// cannot add to some_number.
}


//----------------------------------------
//					MATCH
//----------------------------------------
//	-	Match is a control flow
//	-	After => is code to run, can {} or call function or simply return a value.
//	-	Match is exhaustive, when matching against anything, needs to attempt to match for all.
//	-	Can use "other" for non_covered options if they same behaviour.

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
	let test = value_in_cents(Coin::Quarter(UsState::Alabama));

	println!("{test}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
			println!("In Penny !");
			1
		}
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		}
    }
}

// another example:
fn main() {
	let dice_roll = 9;
	match dice_roll {
		3 => do_x(),
		6 => do_y(),
		_ => do_rest(),				// catch_all like "other", but doesn't bind the value. Use if value's useless
		_ => (),					// if you want to do nothing, () is called unit and is that.
		// other => do_rest(),
	}
}

//----------------------------------------
//				  IF LET
//----------------------------------------
//	-	Used when let + match is too wordy.
//	-	Loses exhaustive check of match

	let config_max = Some(3u8);
	match config_max {
		Some(max) => println!("The maximum is configured to be {}", max),
		_ => (),
	}
// -- TURNS INTO--
	let config_max = Some(3u8);
	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}
