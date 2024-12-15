use common::grid::{Direction, Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let (mut grid, mut position, movements) = parse(input, false);
    for movement in movements {
        let (_, new_position) = move_next(&mut grid, position, movement, true);
        position = new_position;
    }
    grid.iter()
        .filter_map(|(position, val)| {
            if val == 'O' {
                return Some(100 * position.0 + position.1);
            }
            None
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (mut grid, mut position, movements) = parse(input, true);
    for movement in movements {
        let (_, new_position) = move_next(&mut grid, position, movement, true);
        position = new_position;
    }
    grid.iter()
        .filter_map(|(position, val)| {
            if val == '[' {
                return Some(100 * position.0 + position.1);
            }
            None
        })
        .sum()
}

// move_next returns true if the movement was possible and the next point the object moved to
fn move_next(
    grid: &mut Grid<char>,
    position: Point,
    direction: Direction,
    move_if_possible: bool,
) -> (bool, Point) {
    let current = grid.get(position).unwrap();
    match current {
        '#' => (false, position), // can't be moved
        '.' => (true, position),  // nothing to move, can reach this spot
        c @ ('[' | ']') if direction == Direction::Up || direction == Direction::Down => {
            let next_pos = grid
                .next_position(position, direction)
                .expect("Impossible to move off grid");
            let (other_position, other_c) = match_brace(position, c);
            let other_next_pos = grid
                .next_position(other_position, direction)
                .expect("Impossible to move off grid");

            // Call recursively, check if move possible
            let (move_possible, _) = move_next(grid, next_pos, direction, false);
            // Call recursively, check if other move possible
            let (other_move_possible, _) = move_next(grid, other_next_pos, direction, false);
            if move_if_possible && move_possible && other_move_possible {
                move_next(grid, next_pos, direction, true);
                move_next(grid, other_next_pos, direction, true);
                grid.set(position, '.');
                grid.set(next_pos, c);
                grid.set(other_position, '.');
                grid.set(other_next_pos, other_c);
                return (true, next_pos);
            }
            (move_possible && other_move_possible, position)
        }
        'O' | '@' | '[' | ']' => {
            let next_pos = grid
                .next_position(position, direction)
                .expect("Impossible to move off grid");

            // Call recursively, move if successful
            let (move_possible, _) = move_next(grid, next_pos, direction, move_if_possible);
            if move_if_possible && move_possible {
                grid.set(position, '.');
                grid.set(next_pos, current);
                return (true, next_pos);
            }
            // No movement, return old position
            (move_possible, position)
        }
        _ => unreachable!("Invalid character"),
    }
}

fn match_brace(position: Point, c: char) -> (Point, char) {
    match c {
        '[' => ((position.0, position.1 + 1), ']'),
        ']' => ((position.0, position.1 - 1), '['),
        _ => unreachable!("Unexpected char"),
    }
}

fn parse(input: &str, wide: bool) -> (Grid<char>, Point, Vec<Direction>) {
    let (grid, movements) = input.split_once("\n\n").expect("Malformed input");
    let grid = if wide {
        let wider: String = grid
            .chars()
            .map(|c| match c {
                '#' => "##",
                '.' => "..",
                'O' => "[]",
                '@' => "@.",
                '\n' => "\n",
                _ => unreachable!("Unrecognised char"),
            })
            .collect();
        Grid::construct(&wider, |x| x)
    } else {
        Grid::construct(grid, |x| x)
    };

    let starting_position = grid.search('@').expect("No robot found");

    let movements = movements
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '\n' => None,
            _ => unreachable!("Invalid character"),
        })
        .collect();

    (grid, starting_position, movements)
}

common::aoctest!(10092, 1457740, 9021, 1467145);
