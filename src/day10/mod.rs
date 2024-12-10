use std::collections::VecDeque;

use ahash::HashSetExt;

use crate::grid::{Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let grid = Grid::construct(input, |x| (x as u8 - '0' as u8));
    grid.iter()
        .filter_map(|(point, val)| {
            if val == 0 {
                return Some(num_paths_bfs(&grid, point, 9));
            }
            None
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let grid = Grid::construct(input, |x| x);
    0
}

fn num_paths_bfs(grid: &Grid<u8>, start: Point, end: u8) -> usize {
    let mut seen = ahash::HashSet::with_capacity(grid.rows*grid.columns);
    let mut q = VecDeque::with_capacity(100);
    q.push_back(start);
    let mut result = 0;
    while let Some(point) = q.pop_front() {
        let current_val = grid.get(point).unwrap();
        if current_val == end {
            result += 1;
        }
        let (up, right, down, left) = grid.adjacent(point);
        if let Some(val) = up.and_then(|p| grid.get(p)) {
            let p = up.unwrap();
            if val == current_val + 1 && !seen.contains(&p) {
                q.push_back(p);
                seen.insert(p);
            }
        }
        if let Some(val) = right.and_then(|p| grid.get(p)) {
            let p = right.unwrap();
            if val == current_val + 1 && !seen.contains(&p) {
                q.push_back(p);
                seen.insert(p);
            }
        }
        if let Some(val) = down.and_then(|p| grid.get(p)) {
            let p = down.unwrap();
            if val == current_val + 1 && !seen.contains(&p) {
                q.push_back(p);
                seen.insert(p);
            }
        }
        if let Some(val) = left.and_then(|p| grid.get(p)) {
            let p = left.unwrap();
            if val == current_val + 1 && !seen.contains(&p) {
                q.push_back(p);
                seen.insert(p);
            }
        }
    }
    result
}

crate::aoctest!(36, 501, 1234, 1234);