use std::{cmp::Ordering, collections::BinaryHeap};

use ahash::{AHashMap, AHashSet};
use common::grid::{Direction, Grid, Point};

#[allow(clippy::doc_lazy_continuation)]
/// Note on performance:
/// 1. Started at 84ms each, very slow.
/// 2. Reduced to 58ms each (-30%) by changing from HashSet to Vector.
/// 3. Reduced to 23ms each (-60%) by avoiding clones if there is only one path out of this point.
/// 4. Reduced Part 1 to 6.8ms (-70%) by not tracking the path when we're only calculating cost (const generic JUST_COST).
/// 5. Reduced Part 1 to 2.9ms (-57%) and Part 2 to 8.7ms (-62%) by improving the "seen" check. Less time spent on bad paths now.
/// 6. Reduced Part 2 to 6.2ms (-28%) by further reducing clones of the path vector.
/// I think that's the best I can do - 2.8ms and 6.2ms.
/// The code is unreadable right now, so even if I came back later I may not understand what's going on here haha.
#[inline]
pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    dijkstra_path::<true>(grid, start, 'E')
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    dijkstra_path::<false>(grid, start, 'E')
}

fn dijkstra_path<const JUST_COST: bool>(grid: Grid<char>, start: Point, end_char: char) -> usize {
    let mut q = BinaryHeap::with_capacity(100);
    let mut seen: AHashMap<(usize, usize), u32> = AHashMap::with_capacity(grid.rows * grid.columns);
    let start_right = Path {
        point: start,
        cost: 0,
        direction: Direction::Right,
        covered: Vec::with_capacity(100),
    };
    q.push(start_right);
    let mut paths = Vec::with_capacity(5);
    while let Some(mut path) = q.pop() {
        let previous_cost = seen.entry(path.point).or_insert(path.cost);
        if *previous_cost < path.cost && *previous_cost + 1000 != path.cost {
            // We've reached this point through a superior path
            continue;
        }

        // We don't need to track paths if we're only calculating cost
        if !JUST_COST {
            path.covered.push(path.point);
        }

        // We've reached the end_char 'E' on this path.
        if let Some(c) = grid.get(path.point)
            && c == end_char
        {
            if JUST_COST {
                return path.cost as usize;
            }
            paths.push(path);
            continue;
        }

        // Try moving in all directions and return the (next point, cost, and direction).
        let next: Vec<_> = Direction::ALL_DIRECTIONS
            .into_iter()
            .filter_map(|direction| {
                let p = grid.next_position(path.point, direction)?;
                let val = grid.get(p)?;
                if val == '#' || path.direction.opposite() == direction {
                    return None;
                }
                if path.direction == direction {
                    Some((p, 1, direction))
                } else {
                    Some((p, 1001, direction))
                }
            })
            .collect();

        if !next.is_empty() {
            let tmp = Path {
                point: next[0].0,
                cost: path.cost + next[0].1,
                direction: next[0].2,
                covered: path.covered, // We reuse the path that got us to this point
            };
            for n in next.into_iter().skip(1) {
                q.push(Path {
                    point: n.0,
                    cost: path.cost + n.1,
                    direction: n.2,
                    covered: tmp.covered.clone(), // Only clone if there's more than one path out
                });
            }
            q.push(tmp);
        }
    }
    if JUST_COST {
        return 0;
    }
    let cost = paths[0].cost;
    let mut all_good_points = AHashSet::with_capacity(100);
    for path in paths.iter().filter(|path| path.cost == cost) {
        for point in &path.covered {
            all_good_points.insert(point);
        }
    }
    all_good_points.len()
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    point: Point,
    cost: u32,
    direction: Direction,
    covered: Vec<Point>,
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
