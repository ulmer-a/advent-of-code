pub fn main(input: String) -> (u64, u64) {
    let masses: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let fuel = masses.iter().map(|mass| mass / 3 - 2).sum();
    let full_fuel = masses.iter().map(|mass| full_fuel(*mass)).sum();

    (fuel, full_fuel)
}

fn full_fuel(mass: u64) -> u64 {
    match (mass / 3).checked_sub(2).unwrap_or(0) {
        x if x == 0 => x,
        x => x + full_fuel(x),
    }
}
