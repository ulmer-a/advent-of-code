pub fn main(input: String) {
    let (mut x1, mut depth1) = (0u64, 0u64);
    let (mut x2, mut depth2, mut aim2) = (0u64, 0u64, 0u64);

    for line in input.lines() {
        let mut line_iter = line.split(" ").into_iter();
        let command = line_iter.next().unwrap();
        let value = line_iter.next().unwrap().parse::<u64>().unwrap();

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

    println!("result task 1 = {}", x1 * depth1);
    println!("result task 2 = {}", x2 * depth2);
}
