// One way of ensuring safe concurrency is message passing, where threads communicate by sending each other messages containing data.
// To accomplish this, Rust's standard library provides an implementation of channels.
// A channel is a general programming concept by which data is sent from one thread to another.

// A channel has two halves: a transmitter and a receiver.
// One part of the code calls methods on the transmitter with the data to be sent
// Another part checks the receiving end for arriving messages.
// A channel is said to be closed if either the transmitter or receiver half is dropped

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/*
fn main() {
    // Create a channel with the mpsc::channel() function
    // mpsc stands for multiple producer, single consumer
    // A channel can have multiple sending ends that produce values but only one receiving end that consume those values
    // The mpsc::channel() function returns a tuple, the first element is the sending end - the transmitter
    // The second element is the receiving end -- the receiver
    // Note that channels can only be used with one type at a time
    // (e.g. after sending a string through a channel, you can only send strings through the same channel, use an enum if you want to send multiple types)
    // Side note: the tuple being returned is destructured here
    let (tx, rx) = mpsc::channel();

    // Spawning a thread that will take ownership of the transmitter `tx` via `move`
    // The spawned thread needs to won the transmitter to be able to send messages to the channel
    thread::spawn(move || {
        let val = String::from("hi");
        // The transmitter has a send() method that takes the value to send
        // The `send` method returns a Result<T,E> type,
        // If the receiver has already been dropped, the send() operation will return an error
        tx.send(val).unwrap();
    });

    // Main thread gets the sent value from the receiver
    // The receiver has two methods: recv() and try_recv()
    // recv() will block the main thread's execution and wait until a value is sent through the channel
    // Once a value is received, recv() will return a Result<T, E>.
    // When the transmitter closes, recv() will return an error to signal that no more values will be coming
    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
 */

// The try_recv() method doesn't block, and will return a Result<T,E> immediately.
// It will return an Ok() variant holding a message if one is available
// and an Err variant if there aren't any messages at that time

//
// Channels and Ownership Transference
//

// If we tried to use values sent through a channel, that value could be modified or dropped by the other thread before the original thread can use it again
// Rust will throw a compile time error if we try to do that
/*
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // The send() function takes ownership of its parameters, so we can't use 'val' again
        //println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
*/

//
// Sending Multiple values and Seeing the Receiver Waiting
//

/*
fn main() {
    let (tx, rx) = mpsc::channel();

    // The spawned thread will now send multiple messages with 1 second delays
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // Iterate over the vector of strings
        for val in vals {
            // Send the individual string
            tx.send(val).unwrap();
            // Pause for 1 second
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Treat `rx` as an iterator, for each value received, print it
    // When the channel is closed, iteration will end
    for received in rx {
        // We don't have any code that pauses or delays in the `for` loop of the main thread, so we will see each string printed with 1 second delay
        println!("Got: {}", received);
    }
}
*/

//
// Creating Multiple Producers by Cloning the Transmitter
//

fn main() {
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter to allow for multiple producers
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        // Will print out messages from both threads interleaved
        println!("Got: {}", received);
    }
}
