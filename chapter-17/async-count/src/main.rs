use std::time::Duration;

fn main() {
    // This version stops as soon as the for loop in the body of the main async block finishes
    // The task spawned by spawn_task is shut down when the main function ends
    /*
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
    */

    /*
    trpl::run(async {
        // Use a join handle to wait for the spawned task to finish
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // Use `await` to block the main task until the spawned task is finished
        handle.await.unwrap();
    });
    */

    // trpl::join() will take two futures and return a single new future whose output is a tuple with the output of each future once both complete
    // Note that the trpl::join() is fair, meaning it checks each future equally often, alternating between them
    // Also note that runtimes do not have to guarantee fairness
    /*
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
    */
}
