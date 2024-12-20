use ahash::{AHashMap, AHashSet};
use common::grid::{Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    dfs(&grid, start)
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    dfs(&grid, start)
}

fn dfs(grid: &Grid<char>, start: Point) -> usize {
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

    #[allow(unused_variables)]
    let mut results = 0;
    let mut temp = AHashSet::with_capacity(path.len());
    for (point, path_distance) in path.iter() {
        for i in 0..21 {
            for j in 0..21 - i {
                if let Some(other_distance) = path.get(&(point.0 + i, point.1 + j))
                    && path_distance - other_distance >= 50
                {
                    // println!("{}", grid::euclid_distance(point, &(point.0 + i, point.1 + j)));
                    temp.insert((*point, (point.0 + i, point.1 + j)));
                    results += 1;
                }
                if point.0 >= i
                    && let Some(other_distance) = path.get(&(point.0 - i, point.1 + j))
                    && path_distance - other_distance >= 50
                {
                    // println!("{}", grid::euclid_distance(point, &(point.0 - i, point.1 + j)));
                    temp.insert((*point, (point.0 - i, point.1 + j)));
                    results += 1;
                }
                if point.1 >= j
                    && let Some(other_distance) = path.get(&(point.0 + i, point.1 - j))
                    && path_distance - other_distance >= 50
                {
                    // println!("{}", grid::euclid_distance(point, &(point.0 + i, point.1 - j)));
                    temp.insert((*point, (point.0 + i, point.1 - j)));
                    results += 1;
                }
                if point.0 >= i
                    && point.1 >= j
                    && let Some(other_distance) = path.get(&(point.0 - i, point.1 - j))
                    && path_distance - other_distance >= 50
                {
                    // println!("{}", grid::euclid_distance(point, &(point.0 - i, point.1 - j)));
                    temp.insert((*point, (point.0 - i, point.1 - j)));
                    results += 1;
                }
            }
        }
    }

    temp.len()
}

fn parse(input: &str) -> (Grid<char>, Point) {
    let grid = Grid::construct(input, |x| x);
    let start = grid.search('S').expect("grid has a start");
    (grid, start)
}

common::aoctest!(44, 1499, 284, 1234);
