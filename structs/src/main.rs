//	-	Here we use String type instead of &str (slice) because we'd need to use lifetimes
//	-	Introduction to lifetimes is later, so we use String
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

// For user2, we're only changing the email, and copying everything else from user1
// the ..user1 syntax is smart, change what you want, it will copy whatever you didn't
// HOWEVER: here user1 is not usable after assigning
fn	main() {
    let mut user1 = build_user(String::from("Default"), String::from("Default"));
	let user2 = User {
		email: String::from("user2@email.net"),
		..user1
	};

	// user1.email = String::from("Test");
	println!("user1 email: {}\nuser2 email: {}", user1.email, user2.email);
}

// shorthand email and username ONLY WORKS BECAUSE ITS THE SAME AS IN STRUCT
// it is EXPECTING a username and email, so keep the names the same and you don't need username: username
fn	build_user(email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}


//-----------------------------------------------
//				TUPLE STRUCTS
//-----------------------------------------------
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
//	-	Here we can use Color and Point as new types. Color != Point (different types)
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//	-	This is just an introduction to structs with no field.
//	-	Can define a trait to AlwaysEqual. Will see later.
struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
}

// //--------------------------------------------------------------------
// //	-	QUICK PROJECT USING STRUCTS TO CALCULATE SIZE OF RECTANGLE
// //--------------------------------------------------------------------
// //	-	This is for  the println!() call at the end. {:?} needs derive(Debug).
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
	let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
	let rect2 = Rectangle {
        width: 30,
        height: 40,
    };
	let rect3 = Rectangle {
        width: 100,
        height: 50,
    };
	dbg!(&rect1);
	if rect1.width() {
		rect1.area();
	}
	let sq = Rectangle::square(10);
	println!("Can rect1 hold rect2? : {}\nCan rect1 hold rect3? : {}", rect1.can_hold(&rect2), rect1.can_hold(&rect3));
	println!("Can rect1 hold square? : {}", rect1.can_hold(&sq));
}

//	-	Implementing a method to a struct, &self is always first argument
//	-	Note, NOT &mut self, self is not mutable when given to area
//	-	Good for organisation, every function related to rect can be implemented and access through rect1.<x>
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn width(&self) -> bool {
		self.width > 0
	}
	fn can_hold(&self, src: &Rectangle) -> bool {
		//	-	Rust just returns the result of that statement
		self.width > src.width && self.height > src.height
	}
	//	-	This does not have &self as first argument, so it's not a method.
	//	-	usage: String::square(n);
	//	-	Mostly used for constructors that return a new instance of struct.
	fn square(n: u32) -> Self {
		Self {
			width: n,
			height: n,
		}
	}
}