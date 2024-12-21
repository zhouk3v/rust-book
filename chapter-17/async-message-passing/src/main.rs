use std::time::Duration;

fn main() {
    //
    // Sending a single message
    //
    /*
    trpl::run(async {
        // Async version of the multiple-producer, single-consumer channel
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        // We do not await the `send` call, since it does not block as the channel is unbounded
        tx.send(val).unwrap();

        // The recv() method produces a future that we need to await
        // Note that this recv() method is non-blocking, instead it hands control back to the runtime until either a message is received or the send channel is closed
        let received = rx.recv().await.unwrap();
        println!("Got {received}");
    });
    */

    //
    // Sending multiple messages
    //
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Put the `tx` and `rx` operations into their own async blocks to execute them seperately

        // Move `tx` into the async block so that it is dropped after the block to close the channel and exit the other async block (and thus the program) gracefully
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                eprintln!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });

    //
    // Cloning the transmitter
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

        // Use join3 to handle three futures at once
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    })
}
