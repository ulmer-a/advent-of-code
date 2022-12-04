use std::ops::RangeInclusive;

pub fn main(input: String) -> (u64, u64) {
    let pairs = input
        .lines()
        .map(|line| into_ranges(line))
        .map(|(r1, r2)| check_contains(r1, r2))
        .map(|b| if b { 1 } else { 0 })
        .sum();

    (pairs, 0)
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

fn check_contains(r1: RangeInclusive<u64>, r2: RangeInclusive<u64>) -> bool {
    if r1.contains(r2.start()) && r1.contains(r2.end()) {
        true
    } else {
        if r2.contains(r1.start()) && r2.contains(r1.end()) {
            true
        } else {
            false
        }
    }
}

#[test]
fn contains() {
    assert_eq!(check_contains(6..=6, 4..=6), true);
    assert_eq!(check_contains(2..=8, 3..=7), true);
    assert_eq!(check_contains(2..=3, 4..=5), false);
}
