Threads:

- Are an older and more supported form of concurrency
- Use more memory
- Come with some overhead for starting up and shutting down
- Are only an option if the OS and Hardware supports them (e.g. some embedded systems do not have an OS at all, so no threads)

In the async model:

- Concurrent operations run on tasks
- Tasks are like threads, but are managed by a runtime rather than the OS

Threads act as a boundary for sets of synchronous operations: concurrency is possible between threads
Tasks act as a boundary for sets of asynchronous operations: concurrency is possible between and within tasks

Futures are an even more granular unit of concurrency, each future may represent a tree of other futures.
That is, the runtime manages tasks, and tasks manage futures

Threads are in some ways a simpler programming model. They run to completion without interruption except by the OS.
In other words, they have no intra-task concurrency.
Threads also have no mechanisms for cancellation

Tasks then give additional control over futures, allowing us to choose where and how to group them.
Note that threads and tasks can be used together (most runtimes are multithreaded by default, and use work stealing to move tasks between threads)

As a rule of thumb:

- If a task is very parallelizable, such as processing a bunch of data where each part can be processed separately, threads are a better choice
- If a task is very concurrent, like handling messages from different sources, async is a better choice
