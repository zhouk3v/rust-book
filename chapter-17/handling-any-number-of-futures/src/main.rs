use std::{future::Future, pin::Pin, thread, time::Duration};

use trpl::Either;

fn main() {
    //
    // Working with any number of futures
    //
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // the join!() macro can take an arbitary number of futures and handles awaiting the futures itself
        // However, this only works if we know the number of futures ahead of time
        // trpl::join!(tx1_fut, tx_fut, rx_fut);

        // The trpl::join_all() function accepts any type which implements the `Iterator` trait
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        trpl::join_all(futures).await;

        // Since `Pin` is a wrapper type to have a single type for the Vec,
        // We can use `Pin` directly on the futures without doing a heap allocation for `Box` through the `std::pin::pin` macro
        // We still need to be explicit about the type of the pinned reference, for Rust to interpret them as dynamic trait objects for use in the Vec
        // Thus, we directly pin each future when we define it
        let tx1_fut = std::pin::pin!(async move {
            // --snip--
        });

        let rx_fut = std::pin::pin!(async {
            // --snip--
        });

        let tx_fut = std::pin::pin!(async move {
            // --snip--
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        // When joining multiple futures with different output types, join!() will work
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");

        // But join_all() will not, because it requires the futures passed in to be all the same type
        // This serves as a tradeoff between a dynamic number of futures with the same type with join_all(), or a set number of futures with different types for join!()
    });

    //
    // Racing futures
    //

    trpl::run(async {
        // Joining futures requires all of them to finish before moving on
        // In contrast, "racing" futures only requires some (e.g. one) future from a set to finish before moving on

        // Use `trpl::race()` to race futures
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
        // race() will run the futures in the order they are passed in (race() is unfair)
        // Other implementations will randomly choose which futures to poll first
        // Regardless, one of the futures will run up to the first `await` point in its body before another task can start

        // Recall that for each await point, Rust will give the runtime a chance to pause the task and switch to another if the future being awaited is not ready
        // The inverse is also true, Rust will only pause async blocks and hands control back to a runtime at an await point, everything between await points is synchronous

        // This means a future will block any other futures from making progress (starvation), which we need to consider for
    });

    //
    // Yielding
    //

    fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("'{name} ran for {ms}ms");
    }

    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            // yield_now() will immediately hand back control to the runtime from the current async block
            // This is a better alternative than using sleep(), since sleep() will always sleep for at least a millisecond
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });

    //
    // Building out own async abstractions
    //

    // We can compose futures together to create new patterns
    // Eg. a `timeout` function

    trpl::run(async {
        async fn timeout<F: Future>(
            future_to_try: F,
            max_time: Duration,
        ) -> Result<F::Output, Duration> {
            match trpl::race(future_to_try, trpl::sleep(max_time)).await {
                Either::Left(output) => Ok(output),
                Either::Right(_) => Err(max_time),
            }
        }

        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
