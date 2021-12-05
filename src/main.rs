use chrono::{self, Datelike};
use std::{env, fs};

mod aoc2020;
mod aoc2021;
mod bitpush;
mod gridrange;

fn main() {
    let now = chrono::offset::Local::now();
    let mut args = env::args().skip(1);

    // parse day number to be executed
    let id = if let Some(arg) = args.next() {
        arg.parse::<usize>().unwrap()
    } else {
        now.day() as usize // default is current day
    };

    // parse year to be executed from
    let year = if let Some(arg) = args.next() {
        arg.parse::<usize>().unwrap()
    } else {
        now.year() as usize // default is current year
    };

    let filename = format!("input/{}/input{}.txt", year, id);
    let input = fs::read_to_string(filename).unwrap();

    println!("executing task from day {}.12.{}", id, year);
    println!();

    if let Some(result) = run(year, id, input) {
        println!("result task 1 = {}", result.0);
        println!("result task 2 = {}", result.1);
    } else {
        println!(
            "day {} in year '{} doesn't exist or hasn't been solved",
            id,
            year % 100
        );
    }
}

fn run(year: usize, day: usize, input: String) -> Option<(u64, u64)> {
    match year {
        2020 => aoc2020::run(day, input),
        2021 => aoc2021::run(day, input),
        _ => None,
    }
}
