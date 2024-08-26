// traits are more like "objects" from other languages than structs or enums, they bundle data and behaviour.
// however differ because can't add data to a trait "object"
// their specific purpose is to allow abstraction across common behavior.
// Generic type parmeters can only be substituted with 1 concrete type at a time.
pub trait Draw {
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<dyn Draw>>,	// Box<dyn Draw> is trait object, can hold any Type inside a Box<> that implements Draw trait
}
impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}
impl Draw for Button {
	fn draw(&self) {
		// code to draw Button
	}
}


// how to implement screen using generic type parameters
// read main.rs for more info about the tradeoffs.
/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/