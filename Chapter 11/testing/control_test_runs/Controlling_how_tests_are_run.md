`cargo test` compiles the code in test mode and runs the resulting test binary

The default behaviour of the binary produced by cargo test is to run all tests in parallel and capture output generated during test runs,
which prevents output from being displayed to make it easier to see the outputs related to the test results. There are command line options to change this behaviour

Some command line optiosn go to `cargo test`, and some go to the resulting test binary.
To separate these two types of arguments, list the arguments that go to `cargo test` followed by the seperator `--` and then the ones that go to the test binary.

Running `cargo test --help` displays options to use with `cargo test`, and running `cargo test -- --help` displays the options to use after the `--` seperator

# Running Tests in Parallel or Consecutively

Multiple tests are ran in parallel using threads, in order for them to finish faster.

Because tests are running at the same time, make sure that the tests don't depend on each other or on any shared state, including a shared enviroment (e.g. current working directory or environment variables)

As an example, say that two tests write to the same output file. When they are run in parallel, one test might fail due to the other test interfering with the output file. To solve this problem, make the tests write to seperate files, or run the tests sequentially

To run tests sequentially or to control the number of threads used, send the `--test-threads` flag and the number of threads you want the test binary to use.

```
cargo test --- --test-threads=1
```

Setting the number of test threads to 1 will tell the program to not use any parallelism, to prevent the tests from interfering with each other if they share state.

# Showing Function Output

By default, if a test passes, everything printed to standard output is captured.

Example: if `println!` is called in a test and the test passes, the output of `println!` will not be shown in the terminal, only a line that indicates that the test passed. If a test fails, then the line that indicates that the test failed along with whatever was printed to standard output will be shown.

To show the printed values for passing tests as well, use the `--show-output` flag

```
cargo test -- --show-output
```

# Running a Subnet of Tests by Name

We can choose what tests to run by passing `cargo test` the name or names of the test(s) to run

## Running single tests

We can pass the name of any test function to `cargo test` to run only that test:

(E.g. to run a test named `one_hundred`)

```
cargo test one_hundred
```

Note that only the first value given to `cargo test` will be used

## Filtering to Run Multiple Tests

We can specify part of a test name, and any test whose name matches that value will be run.

(E.g. to run tests with `add` in their name)

```
cargo test add
```

Note that the module in which a test appears becomes part of the test's name, so we can run all tests in a module by filtering on the module's name

# Ignoring some tests unless specifically requested

Some tests may not need to be run every time (i.e. tests that take a long time)

We can annotate these tests with the `ignore` attribute to exclude them:

```
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

Ignored tests will be listed as `ignored` in the test output

To run only the ignored tests, use the `--ignored` flag

```
cargo test -- --ignored
```

To run all tests regardless of if they are ignored or not, use the `--include-ignored` flag

```
cargo test -- --include-ignored
```
