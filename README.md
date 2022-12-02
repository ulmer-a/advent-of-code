# Advent of Code 2021-2022

My solutions for Advent of Code 2021 and 2022

<https://adventofcode.com/>

## Build and run

Run the exercises using cargo:

* `cargo run` run example for the today's date (only makes sense in advent)
* `cargo run -- <day>` run example for the specified day in the current year
* `cargo run -- <day> <year>` run example for the specified day in the current year

The `--answer` option that must come last will compute the actual
result for the specified date (not just the example).

## Tests

Most exercise programs and utility routines contain a number of
unit tests to verify correctness. Run them using `cargo test`.

## Highlights
* Day 15/2021 implements Dijkstra's shortest path algorithm
