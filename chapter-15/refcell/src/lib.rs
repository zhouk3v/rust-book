//
// A Use Case for Interior Mutability: Mock Objects
//

// One use case for interior mutability is the creation of mock objects for testing, which assert that the correct actions took place.
// Rust doesn't have objects in the same sense as other languages have objects, and Rust doesn't have mock object functionality in the standard library
// However, it is possible to create a struct that serves the same purpose of a mock object

// Example: a library that tracks a value against a maximum value and sends messages based on how close the maximum value the current value is. (LimitTracker)
// Users of the library will be expected to provide the mechanism for sending the messages. (read: a type that implements the Messenger trait)
// In tests for the limit tracker, we'll provide a mock for the message sending mechanism (read, a mock object that implements the Messenger trait)

pub trait Messenger {
    // Note the send method in the Messenger trait,
    // It takes an immutable reference to self and the text of the message
    // The Messenger trait is the interface that the mock object needs to implement such that the mock can be used in the same way a real object is.
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // We want to test the behaviour of the set_value() method
    // However, set_value() doesn't return anything for us to make assertions on
    // We want to be able to say that if we create a LimitTracker with something that implements the Messenger trait, and a particular value for max,
    // when we pass different numbers for value, the messneger is told to send the appropiate message
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// We need a mock object that will only keep track of the messages it is told to send.
// We can create a new instance of the mock object,
// create a LimitTracker that uses the mock object,
// call the set_value method on LimitTracker,
// and then check that the mock object has the messages we expect.

// First attempt to creating a Mock object for messenger (doesn't work)
/*
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Compile-time error thrown here, since the self reference is immutable, but push() for the sent_messages vec needs the self reference to be mutable
            // We can't change the signature of the send() method to be mutable, as it won't match the original send() method signature in the Messenger trait
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // Wrap the vec in sent_messages in a RefCell<T>
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // The self parameter is still an immutable borrow to match the trait method definition
        fn send(&self, message: &str) {
            // Call borrow_mut() on the RefCell<T> to get a mutable reference to the value inside the RefCell<T>
            // We can now call push() on the Vec
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // Call borrow() on the RefCell<T> to get an immutable reference
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

//
// Keeping track of borrows at Runtime with RefCell<T>
//

// To create immutable and mutable references from RefCell<T> instances, use borrow() and borrow_mut() methods respectively.
// borrow() returns the smart pointer type Ref<T>, and borrow_mut() returns the smart pointer type RefMut<T>
// Both types implement Deref, so we can treat them like regular references

// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
// If borrow() is called, RefCell<T> increases its count of how many immutable borrows are active.
// When a Ref<T> value goes out of scope, the count of immutatble borrows goes down by one.
// RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time
// If the above rule is violated, RefCell<T> will cause a panic at runtime

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
*/

// Choosing to catch borrowing errors at runtime rather than compile time might mean mistakes will pop up later down the line (such as in prod)
// The code will also incur a small runtime penalty

