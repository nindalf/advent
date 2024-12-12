use std::collections::VecDeque;

use ahash::AHashSet;
use common::grid::{Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut seen_points = AHashSet::with_capacity(grid.columns * grid.rows);

    grid.iter()
        .map(|(start, colour)| {
            calc_perimeter_and_area::<false>(&grid, &mut seen_points, start, colour)
        })
        .map(|(perimeter, area)| perimeter * area)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    let mut seen_points = AHashSet::with_capacity(grid.columns * grid.rows);

    grid.iter()
        .map(|(start, colour)| {
            calc_perimeter_and_area::<true>(&grid, &mut seen_points, start, colour)
        })
        .map(|(perimeter, area)| perimeter * area)
        .sum()
}

fn calc_perimeter_and_area<const DISCOUNT_ENABLED: bool>(
    grid: &Grid<char>,
    seen_points: &mut AHashSet<Point>,
    start: Point,
    colour: char,
) -> (usize, usize) {
    if seen_points.contains(&start) {
        return (0, 0);
    }

    let mut perimeter = 0;
    let mut perimeter_points = AHashSet::with_capacity(100);
    let area_before = seen_points.len();
    seen_points.insert(start);

    let mut q = VecDeque::with_capacity(100);
    q.push_back(start);
    while let Some(point) = q.pop_front() {
        let adjacents = grid.adjacent(point);
        let mut same_colour_neighbours = 0;
        for point in adjacents {
            if let Some(p) = point
                && let Some(val) = grid.get(p)
                && val == colour
            {
                same_colour_neighbours += 1;
                if !seen_points.contains(&p) {
                    q.push_back(p);
                    seen_points.insert(p);
                }
            }
        }
        perimeter += 4 - same_colour_neighbours;
        perimeter_points.insert(point);
    }
    let area_after = seen_points.len();
    let perimeter = if DISCOUNT_ENABLED {
        perimeter_corners(perimeter_points, grid.rows, grid.columns)
    } else {
        perimeter
    };
    (perimeter, area_after - area_before)
}

fn perimeter_corners(points: AHashSet<Point>, rows: usize, columns: usize) -> usize {
    points
        .iter()
        .map(|point| {
            let up = (point.0 > 0).then(|| points.contains(&(point.0 - 1, point.1)));
            let right = (point.1 + 1 < columns).then(|| points.contains(&(point.0, point.1 + 1)));
            let down = (point.0 + 1 < rows).then(|| points.contains(&(point.0 + 1, point.1)));
            let left = (point.1 > 0).then(|| points.contains(&(point.0, point.1 - 1)));

            match (up, right, down, left) {
                (Some(true), Some(true), Some(true), Some(true)) => {
                    !points.contains(&(point.0 + 1, point.1 + 1)) as usize
                        + !points.contains(&(point.0 - 1, point.1 - 1)) as usize
                        + !points.contains(&(point.0 + 1, point.1 - 1)) as usize
                        + !points.contains(&(point.0 - 1, point.1 + 1)) as usize
                }
                (Some(true), Some(true), Some(true), _) => {
                    !points.contains(&(point.0 + 1, point.1 + 1)) as usize
                        + !points.contains(&(point.0 - 1, point.1 + 1)) as usize
                }
                (Some(true), Some(true), _, Some(true)) => {
                    !points.contains(&(point.0 - 1, point.1 - 1)) as usize
                        + !points.contains(&(point.0 - 1, point.1 + 1)) as usize
                }
                (Some(true), _, Some(true), Some(true)) => {
                    !points.contains(&(point.0 + 1, point.1 - 1)) as usize
                        + !points.contains(&(point.0 - 1, point.1 - 1)) as usize
                }
                (_, Some(true), Some(true), Some(true)) => {
                    !points.contains(&(point.0 + 1, point.1 - 1)) as usize
                        + !points.contains(&(point.0 + 1, point.1 + 1)) as usize
                }
                (Some(true), Some(true), _, _) => {
                    1 + !points.contains(&(point.0 - 1, point.1 + 1)) as usize
                }
                (Some(true), _, Some(true), _) => 0,
                (Some(true), _, _, Some(true)) => {
                    1 + !points.contains(&(point.0 - 1, point.1 - 1)) as usize
                }
                (_, Some(true), Some(true), _) => {
                    1 + !points.contains(&(point.0 + 1, point.1 + 1)) as usize
                }
                (_, Some(true), _, Some(true)) => 0,
                (_, _, Some(true), Some(true)) => {
                    1 + !points.contains(&(point.0 + 1, point.1 - 1)) as usize
                }
                (Some(true), _, _, _) => 2,
                (_, Some(true), _, _) => 2,
                (_, _, Some(true), _) => 2,
                (_, _, _, Some(true)) => 2,
                (_, _, _, _) => 4,
            }
        })
        .sum()
}

fn parse(input: &str) -> Grid<char> {
    Grid::construct(input, |x| x)
}

common::aoctest!(1930, 1377008, 1206, 815788);
