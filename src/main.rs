use chrono::{self, Datelike};
use std::{env, fs, process::exit};

mod aoc2020;
mod aoc2021;
mod aoc2022;
mod bitpush;
mod gridrange;

fn main() {
    let now = chrono::offset::Local::now();
    let mut args = env::args().skip(1);

    // parse day number to be executed
    let id = if let Some(arg) = args.next() {
        arg.parse::<usize>().unwrap()
    } else {
        if now.month() != 12 || now.day() > 25 {
            print!("error: as today is not an advent date, ");
            println!("you'll have to specify a date to run:");
            println!("use:   cargo run -- [day] [year]");
            exit(1);
        }
        now.day() as usize // default is current day
    };

    // parse year to be executed from
    let year = if let Some(arg) = args.next() {
        arg.parse::<usize>().unwrap()
    } else {
        now.year() as usize // default is current year
    };

    println!("executing task from day {}.12.{}", id, year);
    println!();

    let filename = format!("input/{}/input{}.txt", year, id);
    let input = match fs::read_to_string(&filename) {
        Ok(input) => input,
        Err(error) => {
            println!("{}: {}", filename, error);
            println!("maybe you forgot to provide an input file?");
            exit(1);
        }
    };

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
