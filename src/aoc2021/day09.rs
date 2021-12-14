use std::collections::BinaryHeap;

pub fn main(input: String) -> (u64, u64) {
    let grid_size_x = input.lines().next().unwrap().len();
    let grid_size_y = input.lines().count();
    let mut grid = vec![vec![u8::MAX; grid_size_x]; grid_size_y];
    for (y, line) in input.lines().enumerate() {
        for (x, digit_str) in line.chars().enumerate() {
            grid[y][x] = digit_str.to_digit(10).unwrap() as u8;
        }
    }

    let mut max_sum: u64 = 0;
    let mut basin_sizes = BinaryHeap::new();
    for x in 0..grid_size_x {
        for y in 0..grid_size_y {
            if x > 0 && grid[y][x - 1] <= grid[y][x] {
                continue;
            }
            if y > 0 && grid[y - 1][x] <= grid[y][x] {
                continue;
            }
            if x + 1 < grid_size_x && grid[y][x + 1] <= grid[y][x] {
                continue;
            }
            if y + 1 < grid_size_y && grid[y + 1][x] <= grid[y][x] {
                continue;
            }

            let mut visited = vec![];
            visit_basin((x, y), &grid, &mut visited);
            basin_sizes.push(visited.len() as u64);

            max_sum += grid[y][x] as u64 + 1;
        }
    }

    let mut basin_size_product = 1;
    for _ in 0..3 {
        basin_size_product *= basin_sizes.pop().unwrap();
    }

    (max_sum, basin_size_product)
}

fn visit_basin(
    p: (usize, usize),
    grid: &Vec<Vec<u8>>,
    visited: &mut Vec<(usize, usize)>,
) {
    if grid[p.1][p.0] == 9 || visited.contains(&p) {
        return;
    }

    visited.push(p);

    if p.0 > 0 {
        visit_basin((p.0 - 1, p.1), grid, visited);
    }
    if p.1 > 0 {
        visit_basin((p.0, p.1 - 1), grid, visited);
    }
    if p.0 + 1 < grid[0].len() {
        visit_basin((p.0 + 1, p.1), grid, visited);
    }
    if p.1 + 1 < grid.len() {
        visit_basin((p.0, p.1 + 1), grid, visited);
    }
}

#[test]
fn test() {
    let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
    let r = main(input.into());
    assert_eq!(r.0, 15);
    assert_eq!(r.1, 1134);
}
