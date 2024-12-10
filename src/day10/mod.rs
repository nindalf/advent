use std::collections::VecDeque;

use ahash::HashSetExt;

use crate::grid::{Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let grid = Grid::construct(input, |x| (x as u8 - '0' as u8));
    grid.iter()
        .filter_map(|(point, val)| {
            if val == 0 {
                return Some(num_paths_bfs::<false>(&grid, point, 9));
            }
            None
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> usize {
    let grid = Grid::construct(input, |x| (x as u8 - '0' as u8));
    grid.iter()
        .filter_map(|(point, val)| {
            if val == 0 {
                return Some(num_paths_bfs::<true>(&grid, point, 9));
            }
            None
        })
        .sum()
}

fn num_paths_bfs<const OVERLAPPING_ENABLED: bool>(grid: &Grid<u8>, start: Point, end: u8) -> usize {
    let mut seen = ahash::HashSet::with_capacity(grid.rows*grid.columns);
    let mut q = VecDeque::with_capacity(100);
    q.push_back(start);
    let mut result = 0;
    while let Some(point) = q.pop_front() {
        let current_val = grid.get(point).unwrap();
        if current_val == end {
            result += 1;
            continue;
        }
        let adjacents = grid.adjacent(point);
        for point in adjacents {
            if let Some(p) = point && let Some(val) = grid.get(p) {
                if !OVERLAPPING_ENABLED {
                    if val == current_val + 1 && !seen.contains(&p) {
                        q.push_back(p);
                        seen.insert(p);
                    }
                } else {
                    if val == current_val + 1 {
                        q.push_back(p);
                    }
                }
            }
        }
    }
    result
}

crate::aoctest!(36, 501, 81, 1017);