pub fn main(input: String) -> (u64, u64) {
    // each fish has a counter in the range of 0..8 inclusive that
    // represents the time until it gives birth to a new fish. a given
    // value in the 'fish' array represents the amount of fish being
    // assigned the counter value of it's index.
    let mut fish = [0u64; 9];

    for time_to_next_birth in input.split(",").map(|s| s.parse::<usize>().unwrap()) {
        // load the initial population
        fish[time_to_next_birth] += 1;
    }

    simulate_n_generations(80, &mut fish);
    let p80 = fish.iter().sum(); // population after 80 generations

    simulate_n_generations(256 - 80, &mut fish);
    let p256 = fish.iter().sum(); // population after 256 generations

    (p80, p256)
}

fn simulate_n_generations(n: usize, fish: &mut [u64; 9]) {
    for _ in 0..n {
        let new_fish_count = fish[0];
        for time_to_next_birth in 1..9 {
            // decrement the time_to_next_birth for each fish
            fish[time_to_next_birth - 1] = fish[time_to_next_birth];
        }
        fish[6] += new_fish_count;
        fish[8] = new_fish_count;
    }
}

#[test]
fn test() {
    let input = "3,4,3,1,2";
    let result = main(input.into());
    assert_eq!(result.0, 5934);
    assert_eq!(result.1, 26984457539);
}
