# Future

Trait definition for `Future`

```
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

The `Output` assosciated type determines what the `Future` resolves to.

The `poll` method takes a `Pin` reference to its `self` parameter and a mutable reference to a `Context` type and returns a `Poll<Self::Output>` enum

The `Poll` enum

```
enum Poll<T> {
    Ready(T),
    Pending
}
```

The `Pending` variant means that the future is still in progress and the caller will need to check it again later

The `Ready<T>` variant means that the `Future` is done and the `T` value is available

Note that `poll` should not be called after the future has returned `Ready`. Many futures will panic if polled after becoming `Ready`

The `.await` call compiles to code similar to:

```
let hello_fut = hello("async");
loop {
    match hello_fut.poll() {
        Ready(_) => {
            break;
        }
        Pending => {
            // continue
        }
    }
}
```

Note that in the form above with `loop`, `.await` would be blocking. Instead, the "loop" hands off control to a runtime, which will pause work on the future, work on other futures, and then check the future again.

The runtime will know the future is not ready when the future returns `Poll::Pending`.
The runtime will know the future is ready and advances it when `poll` returns `Poll::Ready(Some(message))` or `Poll::Ready(None)`

# Pinning and the Pin and Unpin traits

`Pin` is a wrapper type that only works with other pointer types (references like `&` and `&mut` and smart pointers like `Box` `Rc` and so on).
More specifically, `Pin` only works on types that implement `Deref` or `DerefMut`

`Pin` is not a pointer itself, rather, it's a tool that the compiler uses to upload relevant guarantees by wrapping pointers in the type

Remember that the await points in a future gets compiled in a state machine, in the form of a enum where each await point is a variant

When moving a future (such as pushing it into a data structure or returning them from a function), we move the state machine of that future.
The futures Rust creates for async blocks can end up with references to themselves in the fields of any given variant.
Any object with a reference to itself is unsafe to move, as references store the actual memory address of the thing they refer to.
If you move the data structure itself, you have to update any references to it

If we can guarantee that the data structure does not move in memory, we do not have to update any references.
The borrow checker already does this: you cannot move an item which has any active references to it using safe code

Thus, when we pin a value by wrapping the pointer to the value with `Pin`, it can no longer move, but the pointer to the value can still move.
(E.g. for `Pin<Box<SomeType>>`, we pin the `SomeType` value)

Most types are safe to move around, we only need to pin items that have internal references.

`Unpin` is a marker trait, that informs the compiler that a given type does not need to upload any particular guarantees about whether the value in question can be moved.
The compiler automatically implements `Unpin` for all types where it can prove it is safe. Implementing `Unpin` manually is unsafe because it requires you to upload all the guarantees which make `Pin` and `Unpin` safe yourself for a type with internal references.

# The Stream trait

Note that there is no definition of a `Stream` trait in the standard library as of yet.

Below is the most common definition

```
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

The associated type `Item` is for the type of items produced by the stream.

The method `poll_next` gets items from the stream. The return type `Poll<Option<Self::Item>>` has `Poll` as the outer type since it has to be checked for readiness, and then the inner type is `Option` since it needs to signal whether there are more messages

The `StreamExt` trait provides the `next()` helper method to work with `Stream` easier, it manages the `poll_next()` calls itself:

```
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}
```

`StreamExt` also contains more helper methods available for streams.
`StreamExt` is automatically implement for every type which implements `Stream`, but are seperated out so that the community can iterate on the foundational trait distinctly from the convenience APIs.
