use crate::gridrange::GridRange;
use std::{cmp::Ordering, collections::BinaryHeap};

pub fn main(input: String) -> (u64, u64) {
    let (grid, grid_size) = parse_graph(input);
    let grid2 = expand_grid(&grid, grid_size, 5);

    (
        shortest_path(grid, grid_size).unwrap(),
        shortest_path(grid2, grid_size * 5).unwrap(),
    )
}

/// dijkstra's shortest path (greedy algorithm)
fn shortest_path(grid: Vec<Vec<(u8, usize)>>, grid_size: usize) -> Option<u64> {
    let mut grid = grid; // make mutable

    // create a priority queue with the start node inserted
    let mut queue = BinaryHeap::new();
    grid[0][0].1 = grid[0][0].0 as usize;
    queue.push(GridNode {
        risk: grid[0][0].0 as usize,
        position: (0, 0),
    });

    // the priority queue pop() method will alway return the
    // node with the cheapest total cost we didn't visit yet
    while let Some(u) = queue.pop() {
        let (x, y) = u.position;
        if x + 1 == grid_size && y + 1 == grid_size {
            // if we're at the bottom right, we're done
            return Some(u.risk as u64 - grid[0][0].0 as u64);
        }

        if u.risk > grid[y][x].1 {
            continue;
        }

        // check out all nodes connected to the current node and
        // look at their risks.
        for (adj_x, adj_y) in GridRange::sq_adjacents(x, y, grid_size) {
            let edge_weight = grid[adj_y][adj_x].0;
            let next_node = GridNode {
                position: (adj_x, adj_y),
                risk: u.risk + edge_weight as usize,
            };

            let total_cost = &mut grid[adj_y][adj_x].1;
            if next_node.risk < *total_cost {
                queue.push(next_node);
                *total_cost = next_node.risk;
            }
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct GridNode {
    position: (usize, usize),
    risk: usize,
}

impl Ord for GridNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for GridNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_graph(input: String) -> (Vec<Vec<(u8, usize)>>, usize) {
    let edge_size = input.lines().count();
    let mut grid = vec![vec![(0, usize::MAX); edge_size]; edge_size];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[y][x].0 = ch.to_digit(10).unwrap() as u8;
        }
    }
    (grid, edge_size)
}

fn expand_grid(
    grid: &Vec<Vec<(u8, usize)>>,
    size: usize,
    n: usize,
) -> Vec<Vec<(u8, usize)>> {
    // create a grid that is five times larger and scale the values accordingly
    let mut new_grid = vec![vec![(0, usize::MAX); size * n]; size * n];
    for y in 0..(size * n) {
        for x in 0..(size * n) {
            let addition = (x / size + y / size) as u8;
            let new_value = grid[y % size][x % size].0 + addition;
            new_grid[y][x].0 = if new_value > 9 {
                new_value - 9
            } else {
                new_value
            };
        }
    }
    new_grid
}

#[test]
fn test_main() {
    let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
    let r = main(input.into());
    assert_eq!(r.0, 40);
    assert_eq!(r.1, 315);
}
