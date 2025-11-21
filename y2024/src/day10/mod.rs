use std::collections::VecDeque;

use ahash::HashSetExt;

use common::grid::{Grid, Point};

/// Note on performance: The const generic bool param makes no difference to part 1, because it it is enabled.
/// For part 2 it does make a 4.2% difference: 62.5µs -> 60.3µs. Marginal, but it shows that
/// const generics can make a difference if used correctly.
///
/// The other favourite trick is rayon, which works poorly here. Since both parts execute so quickly
/// the overhead of using rayon is greater than the benefits it provides.
/// With rayon enabled, part 1 is 245.35 µs (+187% regression) and part 2 is 209.35 µs (+235% regression).
///
/// The last trick is reducing re-allocations, which I'm unable to do here. Increasing the capacity of the
/// HashSet and VecDeque have no effect.
///
/// More optimal algo? No, vanilla BFS is definitely the way to go here
/// More optimal data structures? Can't get better than HashSet and VecDeque for vanilla BFS.
#[inline]
pub fn part1(input: &str) -> usize {
    let grid = parse(input);
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
    let grid = parse(input);
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
    let mut seen = ahash::HashSet::with_capacity(grid.rows * grid.columns);
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
            if let Some(p) = point
                && let Some(val) = grid.get(p)
            {
                if !OVERLAPPING_ENABLED {
                    if val == current_val + 1 && !seen.contains(&p) {
                        q.push_back(p);
                        seen.insert(p);
                    }
                } else if val == current_val + 1 {
                    q.push_back(p);
                }
            }
        }
    }
    result
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::construct(input, |x| x as u8 - b'0')
}

common::aoctest!(36, 501, 81, 1017);
