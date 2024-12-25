use ahash::AHashMap;
use common::grid::{Grid, Point};

/// Performance
/// Part 1 - 473µs, not much to do here.
/// Part 2 - 96.7ms initially. Optimised to 37.8ms (-61%) by replacing a HashSet with a simple counter and preventing double counting.
/// I don't think there's much else I could do for part 2. Visiting every point and checking every other point within a euclid distance of 20
/// is the bare minimum.
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
    #[allow(unused_variables)]
    let mut results = 0;
    for (point, path_distance) in path.iter() {
        for i in 0..21 {
            for j in 0..21 - i {
                let euclid_distance = (i + j) as i32;
                if let Some(other_distance) = path.get(&(point.0 + i, point.1 + j))
                    && path_distance - other_distance - euclid_distance >= min_distance
                {
                    results += 1;
                }
                if point.0 >= i
                    && i != 0
                    && j != 0
                    && let Some(other_distance) = path.get(&(point.0 - i, point.1 + j))
                    && path_distance - other_distance - euclid_distance >= min_distance
                {
                    results += 1;
                }
                if point.1 >= j
                    && i != 0
                    && j != 0
                    && let Some(other_distance) = path.get(&(point.0 + i, point.1 - j))
                    && path_distance - other_distance - euclid_distance >= min_distance
                {
                    results += 1;
                }
                if point.0 >= i
                    && point.1 >= j
                    && let Some(other_distance) = path.get(&(point.0 - i, point.1 - j))
                    && path_distance - other_distance - euclid_distance >= min_distance
                {
                    results += 1;
                }
            }
        }
    }

    results
}

fn parse(input: &str) -> (Grid<char>, Point) {
    let grid = Grid::construct(input, |x| x);
    let start = grid.search('S').expect("grid has a start");
    (grid, start)
}

common::aoctest!(44, 1499, 285, 1027164);
