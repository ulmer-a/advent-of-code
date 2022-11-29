# Advent of Code 2021-2022

My solutions for Advent of Code 2021 and 2022

<https://adventofcode.com/>

## Build and run

Run the exercises using cargo:

* `cargo run` run program for the current day (only makes sense in december)
* `cargo run <day>` run program for the specified day in the current year
* `cargo run <day> <year>` run program for the specified date
* `cargo test day_<day>_<year>` run test for the specified date

## Tests

Each exercise program and the utility code contain a number of
unit tests to verify correctness. Run them using `cargo test`.

Also most days provide integration tests that use the example
that is usually given in the task description. Invoke using:

```
cargo test day_<day>_<year>
```

## Highlights
* Day 15/2021 implements Dijkstra's shortest path algorithm
