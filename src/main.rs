use chrono::{self, Datelike};
use serde::Deserialize;
use std::{fs, process::exit};
use structopt::StructOpt;

mod aoc2020;
mod aoc2021;
mod aoc2022;
mod bitpush;
mod gridrange;

#[cfg(test)]
mod tests;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    day: Option<usize>,

    year: Option<usize>,

    #[structopt(long)]
    answer: bool,
}

#[derive(Deserialize)]
struct TestCase {
    year: usize,
    day: usize,
    task1: u64,
    task2: Option<u64>,
}

fn main() {
    let opt = Opt::from_args();
    let now = chrono::offset::Local::now();

    let year = match opt.year {
        None => now.year() as usize,
        Some(year) => year,
    };

    let day = match opt.day {
        None => now.day() as usize,
        Some(day) => day,
    };

    let filename = match opt.answer {
        true => format!("input/{}/input{}.txt", year, day),
        false => format!("input/tests/{}/input{}.txt", year, day),
    };

    let input = match fs::read_to_string(&filename) {
        Ok(input) => input,
        Err(error) => {
            println!("{}: {}", filename, error);
            println!("maybe you forgot to provide an input file?");
            exit(1);
        }
    };

    println!("executing task from day {}.12.{}", day, year);
    println!();

    let result = run(year, day, input).unwrap();

    if opt.answer {
        println!("task 1: {}", result.0);
        println!("task 1: {}", result.1);
    } else {
        let config = fs::read_to_string("input/tests/tests.yaml").unwrap();
        let testcases: Vec<TestCase> = serde_yaml::from_str(&config).unwrap();
        for tc in testcases {
            if tc.day == day && tc.year == year {
                println!("task1: {}, expected {}", result.0, tc.task1);
                if let Some(t2) = tc.task2 {
                    println!("task2: {}, expected {}", result.1, t2);
                }
            }
        }
    }
}

fn run(year: usize, day: usize, input: String) -> Option<(u64, u64)> {
    match year {
        2020 => aoc2020::run(day, input),
        2021 => aoc2021::run(day, input),
        2022 => aoc2022::run(day, input),
        _ => None,
    }
}
