struct BitPushNumber {
    number: u32,
}

impl BitPushNumber {
    fn new() -> BitPushNumber {
        BitPushNumber { number: 0 }
    }

    fn push(&mut self, bit: u32) {
        self.number = (self.number | bit) << 1;
    }

    fn get(&self) -> u32 {
        self.number >> 1
    }
}

pub fn main(input: String) {
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

    println!(
        "g={} * e={} = {}",
        gamma_rate.get(),
        epsilon_rate.get(),
        gamma_rate.get() * epsilon_rate.get()
    );
}
