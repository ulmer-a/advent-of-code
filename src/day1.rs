pub fn main(input: String) {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let mut increasing_measurements = 0;
    let mut last_window = None;
    for i in 2..numbers.len() {
        let current_window = numbers[i - 2] + numbers[i - 1] + numbers[i];
        if let Some(last_window) = last_window {
            if current_window > last_window {
                increasing_measurements += 1;
            }
        }
        last_window = Some(current_window);
    }

    println!("increasing measurements: {}", increasing_measurements);
}
