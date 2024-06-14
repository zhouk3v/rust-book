# Allowing Transference of Ownership Between Threads with `Send`

The `Send` marker trait indicates that ownership of value/types that implement `Send` can be transferred between threads.

Almost all types in Rust implement `Send`, with few exceptions (i.e. `Rc<T>`)
The reason why `Rc<T>` does not implement `Send` is because when cloning a `Rc<T>` instance and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time.

Rust's type system and trait bounds ensure that a `Rc<T>` is never sent across threads unsafely.

Any type composed entirely of `Send` types is automatically marked `Send` as well. Almost all primative types are `Send`, aside from raw pointers.

# Allowing Access from Multiple Threads with `Sync`

The `Sync` marker trait indicates that it is safe for the type to be referenced from multiple threads.

A type `T` is `Sync` if an immutable reference to `T` (`&T`) is `Send`, i.e. a reference to the type can be sent safely to another thread.

Primative types are `Sync` and types only composed of other `Sync` types are also `Sync`

`Sync` is similar to "thread-safe" in other programming languages

The reason for seperate `Send` and `Sync` types is that a type can be one, both, or neither:

- `Rc<T>` is neither `Send` or `Sync`
- `RefCell<T>` and other `Cell<T>` types are `Send` (if `T` is `Send`), but are not `Sync`. A `RefCell<T>` can be sent across a thread boundary, but not accessed concurrently since the runtime borrow checking implementation is not thread-safe
- `Mutex<T>` is `Send` and `Sync`, and can be used to share access with multiple threads.
- The type `MutexGuard<'a,T>` that is returned by `Mutex::lock` is `Sync` (if `T` is `Sync`) but not `Send`.
  - It is not `Send` because some platform mandate that mutexes are unlocked by the same thread that locked them.

# Implementing `Send` and `Sync` Manually is Unsafe

Because types that are made up of `Send` and `Sync` traits are also automatically `Send` and `Sync`, we don't need to implement these traits manually. (They don't even have any methods to implement)

Manually implementing these triats involves unsafe Rust code
