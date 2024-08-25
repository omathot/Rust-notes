use std::{sync::mpsc, thread, time::Duration};

// rust uses 1:1 os thread to code thread. a spawned thread is a new os process.
// all spawned threads exit when main thread exits.
// DETAIL (hasn't happened on macos) -> no guarantees the spawned thread will run at all if not joined. Depends on os

// Using Message Passing to transfer data between threads
// from go: "Do not communicate by sharing memory; instead, share memory by communicating"
// rust std library has channel -> contains a transmitter and receiver.
// River Metaphor: transmitter is upstream, it sends info down the stream, receiver receives it.
// channel is closed if either of the two is dropped.
// mpsc = Multiple Producer, Single Consumer. can have multiple things going down the stream from different points upstream, stream ends same spot.
fn main() {
	// simple_thread_example();
	let (tx, rx) = mpsc::channel();
	let tx1 = tx.clone();

	thread::spawn(move || {
		let vals = vec![
			String::from("Hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];
		for val in vals {
			tx.send(val).unwrap(); // cannot use val in this scope after having sent it down the stream.
			thread::sleep(Duration::from_secs(1));
		}
	});
	thread::spawn(move || {
		let vals = vec![
			String::from("More"),
			String::from("messages"),
			String::from("for"),
			String::from("you"),
		];
		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});
	
	// recv() blocks the thread and waits to receive.
	// try_recv() does not block and returns Ok() containing message or Err if no message at this time.
	for received in rx {		// not calling recv() explicity here, treating rx as iterator.
		println!("Got: {:?}", received);
	}
}


// takes 5 seconds to execute, uncomment in main() if curious what it does.
fn simple_thread_example() {
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("hey! number {i} from spawned thread!");
			thread::sleep(Duration::from_secs_f32(0.5));
		}
	});
	
	for i in 1..5 {
		println!("hey! number {i} from main thread!");
		thread::sleep(Duration::from_secs_f32(0.5));
	}
	handle.join().unwrap(); // if put before main thread for loop, will execute spawned thread in its entirety first.
}