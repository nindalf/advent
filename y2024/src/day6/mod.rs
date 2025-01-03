use ahash::AHashSet;
use common::grid::{Direction, Grid, Point};
use rayon::prelude::*;

#[inline]
pub fn part1(input: &str) -> usize {
    let (grid, initial_position) = parse(input);
    let steps = steps_to_leave_the_grid(&grid, initial_position);
    steps.len() + 1
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, initial_position) = parse(input);
    let steps = steps_to_leave_the_grid(&grid, initial_position);

    steps
        .par_iter()
        .filter(|obstruction| grid_contains_loop(&grid, initial_position, **obstruction))
        .count()
        + 1
}

fn steps_to_leave_the_grid(grid: &Grid<char>, initial_position: Point) -> AHashSet<Point> {
    let mut guard_direction = Direction::Up;
    let mut guard_position = initial_position;
    let mut visited = AHashSet::with_capacity(grid.rows * grid.columns);
    while let Some((next_position, next_direction)) =
        next_valid_position(grid, None, guard_position, guard_direction)
    {
        visited.insert(guard_position);

        guard_position = next_position;
        guard_direction = next_direction;
    }
    visited
}

fn next_valid_position(
    grid: &Grid<char>,
    obstruction: Option<Point>,
    position: Point,
    mut direction: Direction,
) -> Option<(Point, Direction)> {
    while let Some(next_position) = grid.next_position(position, direction) {
        if Some(next_position) == obstruction {
            direction = direction.turn_right();
            continue;
        }
        match grid.get(next_position) {
            Some('.') => {
                return Some((next_position, direction));
            }
            Some('#') => direction = direction.turn_right(),
            _ => unreachable!("invalid character"),
        }
    }
    // Left the grid
    None
}

fn grid_contains_loop(grid: &Grid<char>, initial_position: Point, obstruction: Point) -> bool {
    let mut guard_direction = Direction::Up;
    let mut guard_position = initial_position;
    let mut visited_with_direction = AHashSet::with_capacity(grid.rows * grid.columns);
    while let Some((next_position, next_direction)) =
        teleport_to_next_obstruction(grid, obstruction, guard_position, guard_direction)
    {
        let position_with_direction = (guard_position.0, guard_position.1, guard_direction);
        if visited_with_direction.contains(&position_with_direction) {
            // We're in a loop, quit.
            return true;
        }
        visited_with_direction.insert(position_with_direction);
        guard_position = next_position;
        guard_direction = next_direction;
    }
    false
}

fn teleport_to_next_obstruction(
    grid: &Grid<char>,
    obstruction: Point,
    position: Point,
    direction: Direction,
) -> Option<(Point, Direction)> {
    let predicate = |c: char, p: Point| {
        if c == '#' || p == obstruction {
            return true;
        }
        false
    };
    let pos = grid.search_until(position, direction, predicate);
    pos.map(|p| (p, direction.turn_right()))
}

pub fn parse(input: &str) -> (Grid<char>, Point) {
    let mut grid = Grid::construct(input, |x| x);
    let guard_position = grid.search('^').unwrap();
    grid.set(guard_position, '.');
    (grid, guard_position)
}

common::aoctest!(41, 4647, 6, 1723);
