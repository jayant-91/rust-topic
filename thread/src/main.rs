mod advanced;
use std::{sync::Arc, thread};
// thread -> allows spawning new threads
// sync::Arc -> atomic reference counting pointer, lets multiple threads safely share ownership of data.

pub fn main() {
    // simple_threading();
    advanced::advanced_threading();
}

fn simple_threading() {
    // counter = 1
    let data = Arc::new(vec![1, 2, 3, 4]);
    // vec![1,2,3,4] creates a vector on the heap.
    // Arc::new(...) wraps it in an atomic reference-counted pointer.
    // Arc keeps track of how many owners there are of the same data. When the last owner is dropped, the data is freed.
    // Comment // counter = 1 means there is 1 reference to the data right now.

    // counter = 2
    let data_for_thread = Arc::clone(&data);
    //Arc::clone does not clone the vector itself; it just increments the reference count.
    // Now both data and data_for_thread share ownership of the same vector.
    // Comment // counter = 2 means there are 2 owners now

    let handle = thread::spawn(move || {
        println!("print data in thread: {:?}", data_for_thread);
    });
    // thread::spawn runs the closure in a new thread.
    // move keyword moves data_for_thread into the thread closure.
    // The thread prints the vector.
    // Why move is needed: Threads run independently, so Rust requires ownership to be transferred safely into the thread.

    handle.join().unwrap();
    // join() blocks the main thread until the spawned thread finishes.
    // .unwrap() just handles any possible error.

    println!("Print data in main: {:?}", data);
    // The main thread still has ownership of the original Arc, so it can safely access the same vector.
}
