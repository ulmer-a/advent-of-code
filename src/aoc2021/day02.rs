pub fn main(input: String) -> (u64, u64) {
    let (mut x1, mut depth1) = (0u64, 0u64);
    let (mut x2, mut depth2, mut aim2) = (0u64, 0u64, 0u64);

    for line in input.lines() {
        let (command, b) = line.split_once(" ").unwrap();
        let value = b.parse::<u64>().unwrap();

        match command {
            "up" => depth1 -= value,
            "down" => depth1 += value,
            "forward" => x1 += value,
            _ => {}
        }

        match command {
            "up" => aim2 -= value,
            "down" => aim2 += value,
            "forward" => {
                x2 += value;
                depth2 += aim2 * value;
            }
            _ => {}
        }
    }

    (x1 * depth1, x2 * depth2)
}

#[test]
fn test() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    let result = main(input.into());
    assert_eq!(result.0, 150);
    assert_eq!(result.1, 900);
}
