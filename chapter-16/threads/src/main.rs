// Within a program, you can also have independent parts that run simultaneously
// The features that run these independent parts are called threads

// Spliting the computation between multiple threads to run multiple tasks at the same time can improve performance, but adds complexity
// Because threads run simultaneously, there's no inherent guarantee about the order in which parts of the code on different threads will run
// This causes problems such as
// - Race conditions: threads accessing data or resources in an inconsistent order (resulting in different results betweeen runs)
// - Deadlocks: where threads will wait for each other, prevent them from continuing (Note that Rust doesn't solve this problem for you)
// - Heisenbugs - bugs that are hard to reproduce and fix reliability

// The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.
// There are crates that implement other models of threading that make different tradeoffs to the 1:1 model

//
// Creating a New Thread with `spawn`
//

use std::thread;
use std::time::Duration;

/*
fn main() {
    // Use the thread::spawn() function and pass it a closure containing the code to run in the new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
*/

// When the main thread of a Rust program completes, all spawned threads are shut down, even if they are not finished running
// (in the above example, the spawned thread might not reach 10)
// The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
// The threads may take turns, but that isn't guaranteed, it depends on how the OS schedules the threads

//
// Waiting for All Threads to Finish Using `join` Handles
//

// Due to the main thread ending, it is not guaranteed that the spawned thread will get to run at all
/*
fn main() {
    // To fix this problem, first, save the return value of thread::spawn()
    // The return type of thread::spawn() is `JoinHandle`
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // A JoinHandle is an owned value that, when we call the join() method on it, will wait for the associated thread to finish
    // Calling join() on the handle blocks the thread currently running until the thread represented by the handle terminates.
    handle.join().unwrap();
}
*/

// The two threads will continue alternating,
// but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.

// Moving handle.join() to before the for loop in main will make the main thread wait for the spawned thread to finish and then run the for loop
/*
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
*/

//
// Using `move` Closures with Threads
//

// The `move` keyword is often used with closures passed to thread::spawn(),
// because the closure will then take ownership of the value it uses from the environment
// thus transferring ownership from one thread to another

// To use data from the main thread in the spawned thread, the spawned thread's closure must capture the value it needs

/*
fn main() {
    let v = vec![1, 2, 3];

    // Letting the closure just borrow (immutably or mutably) the value won't work
    // Rust can't tell how long the spawned thread will run, so it doesn't know if the reference to `v` will always be valid
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
*/

// The main thread can hypothetically drop `v` before the spawned thread finishes
// If Rust allowed this code to run, there is a possibility that the spawned thread would be immediately put in the background without running at all
// The spawned thread has a reference to `v` inside, but the main thread immediately drops `v`
// Then, when the spawned thread starts to execute, `v` is no longer valid, so a reference to it is also invalid
/*
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);

    handle.join().unwrap();
}
*/

// To fix this, use the `move` keyword before the closure to force the closure to take the ownership of the values
fn main() {
    let v = vec![1, 2, 3];

    // Letting the closure just borrow (immutably or mutably) the value won't work
    // Rust can't tell how long the spawned thread will run, so it doesn't know if the reference to `v` will always be valid
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Note that the main thread cannot do anything with `v` after

    handle.join().unwrap();
}
