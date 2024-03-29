use crate::bitpush::BitPushNumber;

pub fn main(input: String) -> (u64, u64) {
    let line_length = input.lines().next().unwrap().len();

    let mut zeroes = vec![0u32; line_length];
    let mut ones = vec![0u32; line_length];

    for line in input.lines() {
        for i in 0..line_length {
            match line.chars().nth(i).unwrap() {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
        }
    }

    let most_common = |i: usize| {
        if ones[i] >= zeroes[i] {
            1
        } else {
            0
        }
    };

    // task 1
    let mut gamma_rate = BitPushNumber::new();
    let mut epsilon_rate = BitPushNumber::new();
    for i in 0..line_length {
        let common = most_common(i);
        gamma_rate.push(common);
        epsilon_rate.push(common ^ 1);
    }

    let result = ((gamma_rate.get() * epsilon_rate.get()) as u64, 0);

    println!(
        "g={} * e={} = {}",
        gamma_rate.get(),
        epsilon_rate.get(),
        result.0
    );

    result
}

#[test]
fn test() {
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    let result = main(input.into());
    assert_eq!(result.0, 198);
}
