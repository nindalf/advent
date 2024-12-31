use ahash::AHashMap;
use common::grid::{Grid, Point};
use rayon::prelude::*;

/// Performance
/// Part 1 - 473µs, not much to do here.
/// Part 2 - 96.7ms initially. Optimised to 37.8ms (-61%) by replacing a HashSet with a simple counter and preventing double counting.
/// Alternate solution: store the path in a Vec, where the index is the distance from the beginning.
/// But it doesn't perform better for either part because the number of lookups is much higher. Cache-friendliness doesn't compensate for that.
/// The code is much cleaner though, so I kept it.
/// Part 1 with Vec takes 59.2ms (+12399% regression).
/// Part 2 with Vec is 59.4ms (+57 regression).
/// Coming back much later, I forgot the obvious one - rayon for part 2.
/// Improves performance by 84% from 39ms -> 6.2ms.
/// And another suggestion by maneatingape - you don't need to check all points, just a subset.
/// Improves performance by 27.4% from 6.2ms to 4.5ms.
#[inline]
pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    let path = dfs_path(&grid, start);
    num_cheats_2(&grid, path)
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    let path = dfs_path(&grid, start);
    num_cheats_20(path)
}

fn dfs_path(grid: &Grid<char>, start: Point) -> AHashMap<Point, i32> {
    let mut distance_so_far = 0;
    let mut path = AHashMap::with_capacity(grid.rows * grid.columns);
    path.insert(start, distance_so_far);
    let mut current = start;
    while grid.get(current).unwrap() != 'E' {
        for point_with_val in grid.adjacent_one(current) {
            if let (Some(p), Some('.' | 'E')) = point_with_val
                && !path.contains_key(&p)
            {
                distance_so_far += 1;
                path.insert(p, distance_so_far);
                current = p;
                break;
            }
        }
    }
    path
}

fn num_cheats_2(grid: &Grid<char>, path: AHashMap<Point, i32>) -> usize {
    let min_distance = if path.len() > 1000 { 100 } else { 0 };
    let mut results = 0;
    for (point, distance) in path.iter() {
        for x in grid.adjacent_two(*point) {
            if let (_, Some('#'), Some(p), Some('.' | 'E')) = x
                && let Some(other_distance) = path.get(&p)
                && other_distance - distance > min_distance
            {
                results += 1;
            }
        }
    }
    results
}

fn num_cheats_20(path: AHashMap<Point, i32>) -> usize {
    let min_distance = if path.len() > 1000 { 100 } else { 50 };
    path.par_iter()
        .map(|(point, path_distance)| {
            let mut results = 0;
            for i in 0..21 {
                for j in 0..21 - i {
                    let euclid_distance = (i + j) as i32;
                    if let Some(other_distance) = path.get(&(point.0 + i, point.1 + j))
                        && ((path_distance - other_distance - euclid_distance >= min_distance)
                         || (other_distance - path_distance - euclid_distance >= min_distance))
                    {
                        results += 1;
                    }
                    if point.0 >= i
                        && i != 0
                        && j != 0
                        && let Some(other_distance) = path.get(&(point.0 - i, point.1 + j))
                        && ((path_distance - other_distance - euclid_distance >= min_distance)
                        || (other_distance - path_distance - euclid_distance >= min_distance))
                    {
                        results += 1;
                    }
                }
            }
            results
        })
        .sum()
}

fn parse(input: &str) -> (Grid<char>, Point) {
    let grid = Grid::construct(input, |x| x);
    let start = grid.search('S').expect("grid has a start");
    (grid, start)
}

#[allow(dead_code)]
fn dfs_path_vec(grid: &Grid<char>, start: Point) -> Vec<Point> {
    let mut path = Vec::with_capacity(grid.rows * grid.columns);
    path.push(start);
    let mut current = start;
    loop {
        for point_with_val in grid.adjacent_one(current) {
            if let (Some(p), Some('.')) = point_with_val
                && (path.len() < 2 || path[path.len() - 2] != p)
            {
                path.push(p);
                current = p;
                break;
            }
            if let (Some(p), Some('E')) = point_with_val {
                path.push(p);
                return path;
            }
        }
    }
}

#[allow(dead_code)]
fn num_cheats_vec(path: Vec<Point>, max_cheat_distance: usize, min_distance: usize) -> usize {
    let mut results = 0;
    for (idx_start, point) in path.iter().enumerate() {
        if idx_start + min_distance >= path.len() {
            break;
        }
        for (idx_end, other_point) in path.iter().enumerate().skip(idx_start + min_distance + 1) {
            let euclid_distance = common::grid::euclid_distance(point, other_point);
            if euclid_distance <= max_cheat_distance
                && idx_end - idx_start > euclid_distance
                && idx_end - idx_start - euclid_distance >= min_distance
            {
                results += 1
            }
        }
    }
    results
}

common::aoctest!(44, 1499, 285, 1027164);
