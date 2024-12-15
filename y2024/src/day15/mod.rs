use common::grid::{Direction, Grid, Point};

#[inline]
pub fn part1(input: &str) -> usize {
    let (mut grid, mut position, movements) = parse(input);
    for movement in movements {
        let (_, new_position) = move_next(&mut grid, position, movement);
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
pub fn part2(_input: &str) -> i32 {
    0
}

// move_next returns true if the movement was successful and the next point the object moved to
fn move_next(grid: &mut Grid<char>, position: Point, direction: Direction) -> (bool, Point) {
    let current = grid.get(position).unwrap();
    println!(
        "Got {current} from {:?} while moving in {direction}",
        position
    );
    match current {
        '#' => (false, position), // can't be moved
        '.' => (true, position),  // nothing to move, can reach this spot
        'O' | '@' => {
            let next_pos = grid
                .next_position(position, direction)
                .expect("Impossible to move off grid");

            // Call recursively, move if successful
            let (successfully_moved, _) = move_next(grid, next_pos, direction);
            if successfully_moved {
                grid.set(position, '.');
                grid.set(next_pos, current);
                return (true, next_pos);
            }
            // No movement, return old position
            (false, position)
        }
        _ => unreachable!("Invalid character"),
    }
}

pub fn parse(input: &str) -> (Grid<char>, Point, Vec<Direction>) {
    let (grid, movements) = input.split_once("\n\n").expect("Malformed input");
    let grid = Grid::construct(grid, |x| x);

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

common::aoctest!(2028, 1457740, 1234, 1234);
