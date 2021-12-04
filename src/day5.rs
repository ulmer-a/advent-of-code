use super::gridrange::GridRange;

fn parse_pt(s: &str) -> (usize, usize) {
    let mut it = s.split(",");
    (
        it.next().unwrap().parse::<usize>().unwrap(),
        it.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn draw_grid(input: &str, allow_diagonal: bool) -> u64 {
    const GRID_SIZE: usize = 1000;
    let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];

    let mut solution_count = 0;
    for line in input.lines() {
        let mut pts = line.split(" -> ");
        let p1 = parse_pt(pts.next().unwrap());
        let p2 = parse_pt(pts.next().unwrap());

        for p in GridRange::new(p1, p2, allow_diagonal) {
            let grid_value = &mut grid[p.1][p.0];
            *grid_value += 1;

            if *grid_value == 2 {
                solution_count += 1;
            }
        }
    }

    solution_count
}

pub fn main(input: String) -> (u64, u64) {
    (draw_grid(&input, false), draw_grid(&input, true))
}

#[test]
fn test() {
    let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    let result = main(input.into());
    assert_eq!(result.0, 5);
    assert_eq!(result.1, 12);
}
