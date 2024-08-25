use std::{sync::{Arc, Mutex}, thread};

// channel message passing is similar to single ownership, once a value is sent down the stream, can't use
// in the transmitter thread anymore.
// mutexes (mutual exclusion)
// use Arc<T> instead of Rc<T> for multiple ownership in multithreaded programs.
// A stands for atomic(?) Rc.
// Arc have performance cost over Rc, so only use them when necessary.
// deadlocks can still happen :
// 		These occur when an operation needs to lock two resources and two threads have each acquired one of
// 		the locks, causing them to wait for each other forever.
fn main() {
    let counter = Arc::new(Mutex::new(0));	// notice we don't create it as mut.
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;												// yet here we mut. it's like RefCell
		});
		handles.push(handle);
	}
	for handle in handles {
		handle.join().unwrap();
	}
	println!("Result: {}", *counter.lock().unwrap());
}
