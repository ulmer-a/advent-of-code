pub fn main(input: String) -> (u64, u64) {
    let numbers: Vec<u64> = input
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let max = *numbers.iter().max().unwrap();

    let mut min_cost = [u64::MAX; 2];
    for i in 0..max {
        let mut cost = [0u64; 2];

        for n in numbers.iter() {
            let abs_cost = (*n as i64 - i as i64).abs() as u64;
            cost[0] += abs_cost;
            cost[1] += (abs_cost * (abs_cost + 1)) / 2;
        }

        for i in 0..2 {
            if cost[i] < min_cost[i] {
                min_cost[i] = cost[i];
            }
        }
    }

    (min_cost[0], min_cost[1])
}

#[test]
fn test() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let r = main(input.into());
    assert_eq!(r.0, 37);
    assert_eq!(r.1, 168);
}
