use itertools::Itertools;

pub fn main(input: String) -> (u64, u64) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let mut points = extract_points(points);

    let mut solution_task1 = None;
    for line in folds.lines() {
        let fold_at = line
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let fold_at_x = line.contains("x=");
        for point in points.iter_mut() {
            if fold_at_x {
                if point.0 > fold_at {
                    point.0 -= 2 * (point.0 - fold_at);
                }
            } else {
                if point.1 > fold_at {
                    point.1 -= 2 * (point.1 - fold_at);
                }
            }
        }

        points = points.into_iter().unique().collect();
        if solution_task1.is_none() {
            solution_task1 = Some(points.iter().count());
        }
    }

    plot_points(points);
    (solution_task1.unwrap() as u64, 0)
}

fn plot_points(points: Vec<(u64, u64)>) {
    let max = points.iter().fold((0, 0), |p1, p2| {
        (std::cmp::max(p1.0, p2.0 + 1), std::cmp::max(p1.1, p2.1 + 1))
    });

    let mut grid = vec![vec![' '; max.0 as usize + 1]; max.1 as usize + 1];
    for p in points {
        grid[p.1 as usize][p.0 as usize] = '#';
    }

    for y in 0..max.1 {
        for x in 0..max.0 {
            print!("{}", grid[y as usize][x as usize]);
        }
        print!("\n");
    }
}

fn extract_points(source: &str) -> Vec<(u64, u64)> {
    source
        .lines()
        .map(|line| {
            let p = line.split_once(",").unwrap();
            (p.0.parse::<u64>().unwrap(), p.1.parse::<u64>().unwrap())
        })
        .collect()
}

#[test]
fn test1() {
    let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
    let r = main(input.into());
    assert_eq!(r.0, 17);
}
