use itertools::Itertools;

type Point = (i64, i64, i64);
type ScannerPts = Vec<Point>;

pub fn main(input: String) -> (u64, u64) {
    let _scanners = parse_input(input);

    (0, 0)
}

fn parse_input(input: String) -> Vec<ScannerPts> {
    let mut scanners = vec![];
    for scanner in input.split("\n\n") {
        let points_iter = scanner.lines().skip(1);
        let mut points = Vec::with_capacity(points_iter.clone().count());
        for line in points_iter {
            let xyz: Point = line
                .split(",")
                .map(|n_str| n_str.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            points.push(xyz);
        }
        scanners.push(points);
    }
    scanners
}
