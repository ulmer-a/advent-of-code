use core::slice::Iter;

// find n numbers in a vector that sum up to a desired
// sum and reports the product of those numbers, if found
fn find_n_summing(sum: u64, n: u8, iter: Iter<u64>, interm: u64) -> Option<u64> {
    let mut iter = iter;
    while let Some(n1) = iter.next() {
        if n - 1 > 0 {
            if let Some(n2) = find_n_summing(sum, n - 1, iter.clone(), *n1 + interm)
            {
                return Some(*n1 * n2);
            }
        } else if *n1 + interm == sum {
            return Some(*n1);
        }
    }
    None
}

pub fn main(input: String) -> (u64, u64) {
    let numbers: Vec<u64> =
        input.lines().map(|x| x.parse::<u64>().unwrap()).collect();

    (
        find_n_summing(2020, 2, numbers.iter(), 0).unwrap(),
        find_n_summing(2020, 3, numbers.iter(), 0).unwrap(),
    )
}

#[test]
fn test() {
    let input = "1721\n979\n366\n299\n675\n1456";
    let r = main(input.into());
    assert_eq!(r.0, 514579);
    assert_eq!(r.1, 241861950);
}
