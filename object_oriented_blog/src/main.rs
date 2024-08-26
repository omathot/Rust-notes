use object_oriented_blog::Post;

// built some extra features the book recommends trying:
// 		reject() and need for 2 approve() calls to publish.
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content()); // post is still a draft

    post.request_review();
    assert_eq!("", post.content()); // post has not been approved yet

	post.reject();			// back to draft
	assert_eq!("", post.content()); 

	post.request_review();	// back to PendingReview, can't go from Draft straight to Published
    post.approve(); // count ++
	post.approve(); // count = 2, now it publishes
    assert_eq!("I ate a salad for lunch today", post.content()); // approved so should return content
}