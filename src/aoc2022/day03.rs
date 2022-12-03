use itertools::{Chunk, Itertools};
use std::str::Lines;

pub fn main(input: String) -> (u64, u64) {
    let priority_single = input
        .lines()
        .map(|line| find_misplaced_item_type(line))
        .map(|item| priority(item))
        .sum();

    let priority_groups = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| find_unique_in_lines(chunk))
        .map(|item| priority(item))
        .sum();

    (priority_single, priority_groups)
}

fn find_misplaced_item_type(line: &str) -> char {
    let half_len = line.len() / 2;
    for c1 in line.bytes().take(half_len) {
        for c2 in line.bytes().skip(half_len) {
            if c1 == c2 {
                return c1 as char;
            }
        }
    }

    panic!("There is no item type occuring in both compartements");
}

fn find_unique_in_lines(lines: Chunk<Lines>) -> char {
    fn char_exists_in_all(lines: &Vec<String>, c: char) -> bool {
        for line in lines {
            if !line.contains(c) {
                return false;
            }
        }
        true
    }

    let lines: Vec<String> = lines.map(|line| line.into()).collect();

    for c in 'a'..='z' {
        if char_exists_in_all(&lines, c) {
            return c;
        }
    }
    for c in 'A'..='Z' {
        if char_exists_in_all(&lines, c) {
            return c;
        }
    }

    panic!("cannot find character that occurs in all rucksacks");
}

fn priority(item: char) -> u64 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    if item.is_ascii_uppercase() {
        item as u64 - 64 + 26
    } else if item.is_ascii_lowercase() {
        item as u64 - 96
    } else {
        panic!("Priority undefined for character {}", item);
    }
}

#[test]
fn find_test() {
    assert_eq!(find_misplaced_item_type("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    assert_eq!(
        find_misplaced_item_type("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        'L'
    );
}

#[test]
fn priority_test() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}
