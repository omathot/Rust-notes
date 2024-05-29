
// traits are similar to interfaces -> with some differences.
// define trait, ended with ';' and not defined {}, each type that uses this trait implements its own version.
// can also define a default behaviour for the trait.
// can have multiple methods in the body.
// for no default behaviour
// fn	summarize(&self) -> String;
pub trait Summary {
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("Read more from {}...", self.summarize_author())
	}
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,

}

// implement the default trait for the News struct
// this was before addition of summarize_author
// impl Summary for NewsArticle {}

// implement the trait for the Tweet struct
impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("{}", self.username)
	}

	// not needed with the addition of summarize_author.
	// it will use default implementation of summarize which looks for summarize_author
	// fn	summarize(&self) -> String {
	// 	format!("{}: {}", self.username, self.content)
	// }
}

fn main() {
    let tweet = Tweet {
		username: String::from("Default"),
		content: String::from("Rust is great"),
		reply: false,
		retweet: false,
	};

	// let article = NewsArticle {
	// 	headline: String::from("Pigeons are dying"),
	// 	location: String::from("Wonderland"),
	// 	author: String::from("Mr. Seuss"),
	// 	content: String::from("Pigeons are dying by the thousands, we must act now"),
	// };
	// println!("new article! [{}]", article.summarize());
	println!("1 new tweet: ({})", tweet.summarize());

}

// can use traits as arguments to function.
// Also no need to be specific about what type the item is as long as <Trait> is implemented.
fn notify(src: &impl Summary) {
	println!("Breaking news! {}", src.summarize());
}

// argument src that implements both Summary and Display
// pub fn notify(item: &(impl Summary + Display)) {

// same as above "trait bound" (basically just spelled out)
pub fn notifyy<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}


// conditionally implementing methods based on traits
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Pair will always have new();
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Pairs of a type that implement PartialOrd and Display will also have cmp_display()
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// example from rust std:: library
// Applies method ToString to all items <T> that implement Display trait
impl<T: Display> ToString for T {
    // --snip--
}