use chrono::{self, Datelike};
use std::{env, fs};

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let id = if let Some(arg) = env::args().skip(1).next() {
        arg.parse::<usize>().unwrap()
    } else {
        let now = chrono::offset::Local::now();
        now.day() as usize
    };

    let filename = format!("input/input{}.txt", id);
    let input = fs::read_to_string(filename).unwrap();

    match id {
        1 => day1::main(input),
        2 => day2::main(input),
        3 => day3::main(input),
        4 => day4::main(input),
        _ => panic!("invalid id"),
    }
}
