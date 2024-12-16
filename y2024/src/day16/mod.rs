use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap},
};

use ahash::{AHashMap, AHashSet};
use common::grid::{Direction, Grid, Point};

#[inline]
pub fn part1(input: &str) -> u32 {
    let (grid, start) = parse(input);
    dijkstra_path(grid, start, 'E').0
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    dijkstra_path(grid, start, 'E').1
}

fn dijkstra_path(grid: Grid<char>, start: Point, end_char: char) -> (u32, usize) {
    let mut q = BinaryHeap::with_capacity(100);
    let mut seen: AHashMap<(usize, usize), u32> = AHashMap::with_capacity(grid.rows * grid.columns);
    let start_right = Path {
        point: start,
        cost: 0,
        direction: Direction::Right,
        covered: AHashSet::with_capacity(100),
    };
    q.push(start_right);
    let mut paths = Vec::with_capacity(5);
    while let Some(mut path) = q.pop() {
        println!(
            "Reached {:?} with cost {} while moving in direction {:?}",
            &path.point, path.cost, path.direction
        );
        match seen.entry(path.point) {
            Entry::Occupied(mut entry) => {
                if *entry.get() + 1000 < path.cost {
                    println!(
                        "Quitting {:?} with cost {} because of previous cost {}",
                        &path.point,
                        path.cost,
                        *entry.get()
                    );
                    // We've reached this point through a superior path
                    continue;
                }
                entry.insert(path.cost);
            }
            Entry::Vacant(entry) => {
                entry.insert(path.cost);
            }
        }
        path.covered.insert(path.point);

        if let Some(c) = grid.get(path.point)
            && c == end_char
        {
            paths.push(path);
            continue;
        }

        let [up, right, down, left] = grid.adjacent(path.point);
        if let Some(p) = up
            && let Some(val) = grid.get(p)
            && val != '#'
            && path.direction != Direction::Down
        {
            let cost = if path.direction == Direction::Up {
                1
            } else {
                1001
            };
            q.push(Path {
                point: p,
                cost: path.cost + cost,
                direction: Direction::Up,
                covered: path.covered.clone(),
            });
        }
        if let Some(p) = right
            && let Some(val) = grid.get(p)
            && val != '#'
            && path.direction != Direction::Left
        {
            let cost = if path.direction == Direction::Right {
                1
            } else {
                1001
            };
            q.push(Path {
                point: p,
                cost: path.cost + cost,
                direction: Direction::Right,
                covered: path.covered.clone(),
            });
        }
        if let Some(p) = down
            && let Some(val) = grid.get(p)
            && val != '#'
            && path.direction != Direction::Up
        {
            let cost = if path.direction == Direction::Down {
                1
            } else {
                1001
            };
            q.push(Path {
                point: p,
                cost: path.cost + cost,
                direction: Direction::Down,
                covered: path.covered.clone(),
            });
        }
        if let Some(p) = left
            && let Some(val) = grid.get(p)
            && val != '#'
            && path.direction != Direction::Right
        {
            let cost = if path.direction == Direction::Left {
                1
            } else {
                1001
            };
            q.push(Path {
                point: p,
                cost: path.cost + cost,
                direction: Direction::Left,
                covered: path.covered.clone(),
            });
        }
    }
    let cost = paths[0].cost;
    let mut all_good_points = AHashSet::with_capacity(100);
    for path in paths.iter().filter(|path| path.cost == cost) {
        for point in &path.covered {
            all_good_points.insert(point);
        }
    }
    (cost, all_good_points.len())
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    point: Point,
    cost: u32,
    direction: Direction,
    covered: AHashSet<Point>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> (Grid<char>, Point) {
    let grid = Grid::construct(input, |x| x);
    let start = grid.search('S').expect("valid grids have a start");
    (grid, start)
}

common::aoctest!(11048, 99460, 64, 500);
