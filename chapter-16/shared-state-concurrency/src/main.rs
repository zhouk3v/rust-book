// Shared Memory is another way of handling concurrency

// Compared to channels (which is based off single ownership), shared memory concurrency is askin to multiple ownership,
// multiple threads can access the same memory location at the same time.
// Mutexes are one common concurrency primatives for shared memory.

//
// Using Mutexes to Allow Access to Data from One Thread at a time
//

// A mutex (mutual exclusion) allows only one thread to access some data at any given time
// To access data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock.
// The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.

// There are two rules for mutexes:
// - You must attempt to acquire the lock before using the data
// - When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock
// Rust's type system and ownership rules will take care of locking and unlocking the mutex

//
// The API of Mutex<T>
//

/*
// A mutex in a single-threaded context
use std::sync::Mutex;

fn main() {
    // Create a mutex with the new() associated function
    let m = Mutex::new(5);

    {
        // Use the lock() method to access the data inside the mutex
        // We use the lock() method to acquire the lock.
        // lock() will block the current thread so it can't do any work until it obtains the lock

        // The call to lock would fail if a thread holding the lock panicked, so no one else will be able to get the lock
        // This is why lock returns a Result<T,E>, where we just use unwrap() on in this example
        let mut num = m.lock().unwrap();
        // After acquiring the lock, the return value is treated as a mutable reference to the data inside
        // Note that the type system ensures that the lock is acquired before using the value inside it (`m` is a Mutex<i32>, not i32)
        // The call to lock() returns a smart pointer called `MutexGuard`, wrapped in a `LockResult` (which is handled by unwrap())
        // The `MutexGuard` smart pointer implements Deref to point to the inner data
        // The Drop() implementation of the `MutexGuard` smart pointer releases the lock automatically when a `MutexGuard` goes out of scope (no need to manually unlock it)
        *num = 6;
    }
    // We'll see that the mutex value was changed to 6 after dropping the lock
    println!("m = {:?}", m);
}
*/

//
// Sharing a Mutex<T> Between Multiple threads
//

/*
// This won't work
use std::thread;
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    // Create 10 threads that will increment `counter`
    for _ in 0..10 {
        let handle = thread::spawn(
            move || {
                // Note that the closure moves `counter` into the spawned thread
                // - The compiler will error out here, saying that we can't move ownership of the `counter` mutex into multiple threads
                // - (Not in book) Remember that the closure we pass into the thread will take ownership of any value in its enviroment it uses
                // - In this cause, the closure in the first spawned thread will move ownership of the mutex,
                // - and then an error is thrown when the closure of the second thread tries to move ownership of the mutex too
                // The spawned thread will lock the mutex
                let mut num = counter.lock().unwrap();
                // and then increment the value in the mutex by 1
                *num += 1;
            }, // num goes out of scope here for the spawned thread, and the lock is released
        );
        // Collect all join handles
        handles.push(handle);
    }

    // Call join() on each handles to make sure the threads finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Main thread will print out result
    println!("Result: {}", *counter.lock().unwrap());
}
*/

//
// Multiple Ownership with Multiple Threads
//

// This won't work
/*
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Wrap the mutex in an Rc<T> to try to enable multiple ownership for multiple closures
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            // compiler error here - the trait `Send` is not implemented for Rc<T>
            // Rc<T> is not safe to share across threads.
            // It does not have concurrency primatives to make sure that changes to the reference count can't be interrupted by another thread
            // This can lead to wrong counts, which can result to memory leaks or values being dropped before being used
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
*/

//
// Atomic Reference Counting with Arc<T>
//

// Arc<T> is like Rc<T>, but safe to use in concurrent situations.
// The `a` stands for atomic (meaning it is safe to share across threads), so Arc<T> stands for `atomically reference counted` type

// Note that thread safety comes with a performance penalty, due to the need to enforce guarantees of atomic types
// In single threaded situations, continue using Rc<T>

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Note that there are other atomic types provided by std::sync::atomic that provide safe, concurrent, atomic, access to primative types

//
// Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
//

// Note that `counter` is immutable, but we could get a mutable reference to the value inside of it.
// Mutex<T> provides interior mutability
// Compared to RefCell<T> which allows the mutation of contents inside Rc<T>, Mutex<T> allows mutation of contents inside an Arc<T>

// Also compared to RefCell<T>, where there is a risk of reference cycles, Mutex<T> comes with the risk of creating deadlocks
