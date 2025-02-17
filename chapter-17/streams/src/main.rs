// StreamExt provides more utility functions to streams that are similar to utility functions for Iterators
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    //
    // Streams
    //

    // trpl::run(async {
    //     let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let iter = values.iter().map(|n| n * 2);
    //     let mut stream = trpl::stream_from_iter(iter);

    //     while let Some(value) = stream.next().await {
    //         println!("The value was: {value}");
    //     }
    // });

    // using the filter() method on stream
    // trpl::run(async {
    //     let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let iter = values.iter().map(|n| n * 2);
    //     let mut stream = trpl::stream_from_iter(iter);

    //     let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

    //     while let Some(value) = filtered.next().await {
    //         println!("The value was: {value}");
    //     }
    // });

    //
    // Composing Streams
    //

    // get_messages() function that returns a stream from an async channel
    // fn get_messages() -> impl Stream<Item = String> {
    //     let (tx, rx) = trpl::channel();

    //     let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    //     for message in messages {
    //         tx.send(format!("Message: '{message}'")).unwrap();
    //     }

    //     // ReceiverStream converts the `rx` recevier from the async channel into a Stream object
    //     ReceiverStream::new(rx)
    // }

    // trpl::run(async {
    //     let mut messages = get_messages();

    //     while let Some(message) = messages.next().await {
    //         println!("{message}");
    //     }
    // })

    // Adding a timeout to the stream to return an error if a message does not arrive in time
    // trpl::run(async {
    //     let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

    //     while let Some(result) = messages.next().await {
    //         match result {
    //             Ok(message) => println!("{message}"),
    //             Err(reason) => eprintln!("Problem: {reason:?}"),
    //         }
    //     }
    // });

    // Applying a variable delay to messages in get_messages() to force a timeout error
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        // To sleep between messages, we need to use async
        // Spawn a task to handle the async `sleep` calls
        // Note that we cannot make the get_messages() function itself async, or else it will return a Future<Output = Stream<Item = String>>
        // This would mean that it would send all messages before returning the receiver stream
        // (everything in a future happens linearly, but concurrency happens between futures)
        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;

                // Handle error when sending
                if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                    eprintln!("Cannot send message '{message}': {send_error}");
                    break;
                }
            }
        });

        ReceiverStream::new(rx)
    }

    // trpl::run(async {
    //     let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

    //     while let Some(result) = messages.next().await {
    //         match result {
    //             Ok(message) => println!("{message}"),
    //             Err(reason) => eprintln!("Problem: {reason:?}"),
    //         }
    //     }
    // });

    //
    // Merging Streams
    //

    // A get_intervals() function
    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;

                // Handle error when sending
                if let Err(send_error) = tx.send(count) {
                    eprintln!("Could not send interval {count}: {send_error}");
                    break;
                };
            }
        });

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        // Note that the streams to merge must have the same type, so modify the intervals stream to be of type `Timeout<impl Stream<Item = String>>`
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            // Throttle the intervals stream to avoid overwheming the messages stream
            // Note that this produces a new stream that is polled at the throttle rate
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages
            .merge(intervals)
            // Only take the first 20 items from the stream
            .take(20);
        let mut stream = pin!(merged);
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}
