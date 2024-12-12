use std::collections::VecDeque;

use ahash::AHashSet;
use common::grid::{Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut seen_points = AHashSet::with_capacity(grid.columns*grid.rows);

    grid.iter()
        .map(|(start, colour)| calc_perimeter_and_area(&grid, &mut seen_points, start, colour))
        .map(|(perimeter, area)| perimeter * area)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i32 {
    0
}

fn calc_perimeter_and_area(grid: &Grid<char>, seen_points: &mut AHashSet<Point>, start: Point, colour: char) -> (usize, usize) {
    if seen_points.contains(&start) {
        return (0, 0);
    }
    let area_before = seen_points.len();
    let mut q = VecDeque::with_capacity(100);
    q.push_back(start);
    seen_points.insert(start);
    let mut perimeter = 0;
    while let Some(point) = q.pop_front() {
        let adjacents = grid.adjacent(point);
        let mut same_colour_neighbours = 0;
        for point in adjacents {
            if let Some(p) = point
                && let Some(val) = grid.get(p)
            {
                if val == colour {
                    same_colour_neighbours += 1;
                    if !seen_points.contains(&p) {
                        q.push_back(p);
                        seen_points.insert(p);
                    }
                }
            }
        }
        perimeter += 4 - same_colour_neighbours;
    }
    let area_after = seen_points.len();
    (perimeter, area_after - area_before)
}


fn parse(input: &str) -> Grid<char> {
    Grid::construct(input, |x| x)
}

common::aoctest!(1930, 1234, 1234, 1234);