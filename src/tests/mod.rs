use super::run;
use std::fs;

mod integration2021;

fn run_test(year: usize, day: usize) -> (u64, u64) {
    let input =
        fs::read_to_string(&format!("tests/{}/input{}.txt", year, day)).unwrap();
    run(year, day, input).unwrap()
}
