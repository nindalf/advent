use std::collections::VecDeque;

use ahash::AHashSet;
use common::grid::{Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let corrupted = parse(input);
    let (grid_side, num_corrupted_points) = match corrupted.len() {
        25 => (7, 12),      // test input
        3450 => (71, 1024), // real input
        _ => unreachable!("Unknown input length"),
    };

    let grid_points = vec!['.'; grid_side * grid_side];
    let grid = Grid::manual_construct(grid_points, grid_side, grid_side);

    simple_bfs(
        &grid,
        (0, 0),
        (grid_side - 1, grid_side - 1),
        &corrupted[0..num_corrupted_points],
    )
    .unwrap()
}

#[inline]
pub fn part2(input: &str) -> (usize, usize) {
    let corrupted = parse(input);
    let grid_side = match corrupted.len() {
        25 => 7,    // test input
        3450 => 71, // real input
        _ => unreachable!("Unknown input length"),
    };

    let grid_points = vec!['.'; grid_side * grid_side];
    let grid = Grid::manual_construct(grid_points, grid_side, grid_side);

    // Binary search to find the point at which a path to the end becomes impossible
    let (mut start, mut end) = (0, corrupted.len());
    loop {
        if start == end || start == end - 1 {
            return corrupted[start];
        }
        let idx = (start + end) / 2;
        match simple_bfs(
            &grid,
            (0, 0),
            (grid_side - 1, grid_side - 1),
            &corrupted[0..idx],
        ) {
            Some(_) => {
                // path possible, we need more corrupted points
                start = idx;
            }
            None => {
                // path not possible, we need fewer corrupted points
                end = idx;
            }
        };
    }
}

fn simple_bfs(
    grid: &Grid<char>,
    start: Point,
    end: Point,
    points_to_avoid: &[Point],
) -> Option<usize> {
    let points_to_avoid: AHashSet<Point> = points_to_avoid.iter().copied().collect();

    let size = grid.rows * grid.columns - points_to_avoid.len();
    let mut seen = AHashSet::with_capacity(size);
    let mut q = VecDeque::with_capacity(size);
    q.push_back(Path {
        point: start,
        distance_so_far: 0,
    });
    while let Some(path) = q.pop_front() {
        if path.point == end {
            return Some(path.distance_so_far);
        }
        let adjacents = grid.adjacent(path.point);
        for point in adjacents {
            if let Some(p) = point
                && !points_to_avoid.contains(&p)
                && !seen.contains(&p)
            {
                q.push_back(Path {
                    point: p,
                    distance_so_far: path.distance_so_far + 1,
                });
                seen.insert(p);
            }
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    point: Point,
    distance_so_far: usize,
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let corrupted = input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    corrupted
}

common::aoctest!(22, 310, (6, 1), (16, 46));
