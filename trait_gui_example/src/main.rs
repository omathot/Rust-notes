use trait_gui_example::{Draw, Screen, Button};

struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>,
}
impl Draw for SelectBox {
	fn draw(&self) {
		// code to draw SelectBox
	}
}

// we can draw any type that implements the trait draw, and we are not concerned with what that type is.
// Generic type parameters uses "static dispatch", traits use "dynamic dispatch"
fn main() {
	let screen = Screen {
		components: vec![
			Box::new(SelectBox {
				width: 75,
				height: 10,
				options: vec![
					String::from("yes"),
					String::from("maybe"),
					String::from("no"),
				],
			}),
			Box::new(Button {
				width: 50,
				height: 10,
				label: String::from("Hi"),
			}),
		],
	};
	screen.run();
}

// if the library implemented Screen using generic type parameters, it would only be able to hold 1 type.
// if the collection of components is only ever homogenous, generic type parameters are fine.
// Here a user defined struct X can also fit in screen because it only needs to implement Draw.