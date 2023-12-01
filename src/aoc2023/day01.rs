use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn main(input: String) -> (u64, u64) {
    let task1 = input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u64);

            let first = digits.clone().next().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum();

    let task2 = input
        .lines()
        .map(|line| {
            let first = find_first(line).unwrap();
            let last = find_last(line).unwrap();
            first * 10 + last
        })
        .sum();

    (task1, task2)
}

lazy_static! {
    static ref WORDS: HashMap<&'static str, u64> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

fn find_first(s: &str) -> Option<u64> {
    let mut min_index = s.len();
    let mut min_value = None;

    for (word, value) in WORDS.iter() {
        if let Some(index) = s.find(word) {
            if index < min_index {
                min_index = index;
                min_value.replace(*value);
            }
        }
    }

    for (i, c) in s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c.to_digit(10) {
            Some(digit) => Some((i, digit as u64)),
            None => None,
        })
    {
        if i < min_index {
            min_index = i;
            min_value.replace(c as u64);
        }
    }

    min_value
}

fn find_last(s: &str) -> Option<u64> {
    let mut max_index = 0;
    let mut max_value = None;
    for (word, value) in WORDS.iter() {
        if let Some(index) = s.rfind(word) {
            if index >= max_index {
                max_index = index;
                max_value.replace(*value);
            }
        }
    }

    for (i, c) in s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c.to_digit(10) {
            Some(digit) => Some((i, digit as u64)),
            None => None,
        })
    {
        if i >= max_index {
            max_index = i;
            max_value.replace(c);
        }
    }

    max_value
}
