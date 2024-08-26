pub struct Post {
	state: Option<Box<dyn State>>,	// Option here is to allow us to call take() to consume the state, make it None, and change it.
	content: String,
	/*
	We need to set state to None temporarily rather than setting it directly with code like
	self.state = self.state.request_review(); to get ownership of the state value.
	This ensures Post can’t use the old state value after we’ve transformed it into a new state.
	*/
}
impl Post {
	pub fn new() -> Post {
		Post {
			state: Some(Box::new(Draft {})),
			content: String::new(),
		}
	}
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}
	// Now we can start seeing the advantages of the state pattern: the request_review method on Post is
	// the same no matter its state value. Each state is responsible for its own rules.
	pub fn request_review(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.request_review())
		}
	}
	pub fn approve(&mut self) {	// request_review() and approve() are essentially the same function, can use macros
		if let Some(s) = self.state.take() {
			self.state = Some(s.approve());
		}
	}
	pub fn reject(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.reject());
		}
	}

	pub fn content(&self) -> &str {
		self.state.as_ref().unwrap().content(self)
	}
}


trait State {
	// take note of signature: only valid when called on Box<> type
	// takes ownership of what's in Box, consumes it to change it
	fn request_review(self: Box<Self>) -> Box<dyn State>;
	fn approve(self: Box<Self>) -> Box<dyn State>;
	fn reject(self: Box<Self>) -> Box<dyn State>;
	// need lifetime, returning part of post as ref, need guarantee that original post lives long enough
	fn content<'a>(&self, post: &'a Post) -> &'a str { // define default behaviour, only Published overwrites
		""
	}
}

struct Draft{}
impl State for Draft {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		Box::new(PendingReview {
			count: 0,
		})
	}
	fn approve(self: Box<Self>) -> Box<dyn State> {
		self	// can't approve a draft, needs review first, so return self
	}
	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}
}

struct PendingReview {
	count: i8,
}
impl State for PendingReview {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}
	fn approve(mut self: Box<Self>) -> Box<dyn State> {
		self.count += 1;
		if self.count >= 2 {
			Box::new(Published {})
		} else {
			self
		}
	}
	fn reject(self: Box<Self>) -> Box<dyn State> {
		Box::new(Draft {})
	}
}

struct Published{}
impl State for Published {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self // should stay published
	}
	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}
	fn content<'a>(&self, post: &'a Post) -> &'a str {
		&post.content
	}
	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}
}