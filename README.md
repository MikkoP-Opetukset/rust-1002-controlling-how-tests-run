# 1002: Controlling How Tests Are Run

Material for the Rust Programming course @ SAMK

## References

This demo follows concepts and selected examples from
[The Rust Programming Language, Chapter 11.2: Controlling How Tests Are Run](https://doc.rust-lang.org/book/ch11-02-running-tests.html).

The Rust Book is available under the MIT License and Apache License, Version 2.0.

## Running the tests

Run these commands from this demo's directory:

- `cargo test` runs the regular test suite. Passing-test output is captured and
  hidden; the extended test is ignored by default.
- `cargo test save_file_round_trips` runs only tests whose names contain
  `save_file_round_trips`.
- `cargo test -- --list` lists the tests without running them.
- `cargo test -- --show-output` runs the tests and shows shows captured  output 
  after the tests pass.
- `cargo test -- --nocapture` runs the tests and prints their outputs 
  immediately as they runs.
- `cargo test -- --test-threads=1` runs tests one at a time instead of in
  parallel.
- `cargo test -- --ignored` runs only the ignored extended test.
- `cargo test -- --include-ignored` runs the regular tests and the ignored
  extended test.
