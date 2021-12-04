pub fn main(input: String) -> (u64, u64) {
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

    (0, increasing_measurements)
}

#[test]
fn test() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    let result = main(input.into());
    assert_eq!(result.1, 5);
}
