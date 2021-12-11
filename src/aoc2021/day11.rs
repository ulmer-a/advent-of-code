use crate::gridrange::GridRange;

const GRID_SIZE: usize = 10;
type Grid = [[u16; GRID_SIZE]; GRID_SIZE];

pub fn main(input: String) -> (u64, u64) {
    let mut grid = [[0u16; GRID_SIZE]; GRID_SIZE];
    fill_grid(input, &mut grid);

    let mut flash_count = 0;
    let mut total_flash = None;
    for step in 0..100 {
        let increment = sim_step(&mut grid);
        flash_count += increment;

        if increment >= 100 {
            total_flash = Some(step);
        }
    }

    let mut step = 100;
    while total_flash.is_none() {
        step += 1;
        if sim_step(&mut grid) >= 100 {
            total_flash = Some(step);
        }
    }

    (flash_count, total_flash.unwrap())
}

fn sim_step(grid: &mut Grid) -> u64 {
    // increase energy level by 1
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            grid[y][x] += 1;
        }
    }

    // any octopus with an energy level of 9 + 1 flashes
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if grid[y][x] == 10 {
                flash(x, y, grid);
            }
        }
    }

    // collect number of flashes and clear flashed cells to zero
    let mut flashes = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if grid[y][x] == 0xff {
                flashes += 1;
                grid[y][x] = 0;
            }
        }
    }
    flashes
}

fn flash(x: usize, y: usize, grid: &mut Grid) {
    grid[y][x] = 0xff; // mark this octopus flashed
    for (x_a, y_a) in GridRange::adjacents(x, y, GRID_SIZE) {
        if grid[y_a][x_a] >= 10 {
            continue;
        }

        grid[y_a][x_a] += 1;
        if grid[y_a][x_a] == 10 {
            flash(x_a, y_a, grid);
        }
    }
}

fn fill_grid(input: String, grid: &mut Grid) {
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            grid[y][x] = character.to_digit(10).unwrap() as u16;
        }
    }
}

#[test]
fn test() {
    let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    let r = main(input.into());
    assert_eq!(r.0, 1656);
    assert_eq!(r.1, 195);
}
