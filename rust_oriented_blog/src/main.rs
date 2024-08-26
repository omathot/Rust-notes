use rust_oriented_blog::Post;


// this shows how to implement object_oriented_blog example in a way that leverages Rust compiler
// has all the same functionalities, except double approval is done not with a counter but with another Struct
fn main() {
    let mut post = Post::new(); // return DraftPost

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();	// review to PendingReview
	let post = post.reject();					// reject back to Draft
	let post = post.request_review();	// review to PendingReview
	let post = post.approve();			// approve to PendingApprove
	let post = post.approve();						// approve to Post
	assert_eq!("I ate a salad for lunch today", post.content()); // can now see content of post
}