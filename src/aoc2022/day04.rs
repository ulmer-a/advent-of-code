use std::{collections::HashSet, ops::RangeInclusive};

pub fn main(input: String) -> (u64, u64) {
    let pairs_fully_contained = input
        .lines()
        .map(|line| into_ranges(line))
        .map(|(r1, r2)| contains(r1, r2))
        .map(|b| if b { 1 } else { 0 })
        .sum();

    let pairs_overlapping = input
        .lines()
        .map(|line| into_ranges(line))
        .map(|(r1, r2)| overlaps(r1, r2))
        .map(|b| if b { 1 } else { 0 })
        .sum();

    (pairs_fully_contained, pairs_overlapping)
}

fn into_ranges(line: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let (s1, s2) = line.split_once(",").unwrap();
    (str2range(s1), str2range(s2))
}

fn str2range(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s.split_once("-").unwrap();
    let start = start.parse::<u64>().unwrap();
    let end = end.parse::<u64>().unwrap();
    start..=end
}

fn contains(r1: RangeInclusive<u64>, r2: RangeInclusive<u64>) -> bool {
    let s1: HashSet<u64> = HashSet::from_iter(r1);
    let s2: HashSet<u64> = HashSet::from_iter(r2);
    let intersect_len = s1.intersection(&s2).count();
    s1.len() == intersect_len || s2.len() == intersect_len
}

fn overlaps(r1: RangeInclusive<u64>, r2: RangeInclusive<u64>) -> bool {
    let s1: HashSet<u64> = HashSet::from_iter(r1);
    let s2: HashSet<u64> = HashSet::from_iter(r2);
    s1.intersection(&s2).count() > 0
}

#[test]
fn contains_test() {
    assert_eq!(contains(6..=6, 4..=6), true);
    assert_eq!(contains(2..=8, 3..=7), true);
    assert_eq!(contains(2..=3, 4..=5), false);
}
