use ahash::AHashSet;
use common::grid::{Direction, Grid, Point};
use rayon::prelude::*;

#[inline]
pub fn part1(input: &str) -> usize {
    let (grid, initial_position) = parse(input);
    let steps = steps_to_leave_the_grid(&grid, initial_position, None).unwrap();
    steps.len() + 1
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, initial_position) = parse(input);
    let steps = steps_to_leave_the_grid(&grid, initial_position, None).unwrap();

    steps
        .par_iter()
        .map(|obstruction| steps_to_leave_the_grid(&grid, initial_position, Some(*obstruction)))
        .filter(|optional_steps| optional_steps.is_none())
        .count()
        + 1
}

fn steps_to_leave_the_grid(
    grid: &Grid<char>,
    initial_position: Point,
    obstruction: Option<Point>,
) -> Option<AHashSet<Point>> {
    let mut guard_direction = Direction::Up;
    let mut guard_position = initial_position;
    let mut visited = AHashSet::with_capacity(grid.rows * grid.columns);
    let mut visited_with_direction = AHashSet::with_capacity(grid.rows * grid.columns);
    while let Some((next_position, next_direction)) =
        next_valid_position(grid, obstruction, guard_position, guard_direction)
    {
        visited.insert(guard_position);
        let position_with_direction = (guard_position.0, guard_position.1, guard_direction);
        if visited_with_direction.contains(&position_with_direction) {
            // We're in a loop, quit.
            return None;
        }
        visited_with_direction.insert(position_with_direction);
        guard_position = next_position;
        guard_direction = next_direction;
    }
    Some(visited)
}

fn next_valid_position(
    grid: &Grid<char>,
    obstruction: Option<Point>,
    position: Point,
    mut direction: Direction,
) -> Option<(Point, Direction)> {
    while let Some(next_position) = grid.next_position(position, direction) {
        if obstruction.is_some() && obstruction.unwrap() == next_position {
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

fn parse(input: &str) -> (Grid<char>, Point) {
    let mut grid = Grid::construct(input, |x| x);
    let guard_position = grid.search('^').unwrap();
    grid.set(guard_position, '.');
    (grid, guard_position)
}

common::aoctest!(41, 4647, 6, 1723);
