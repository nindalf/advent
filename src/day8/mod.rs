use ahash::AHashMap;

use crate::grid::Grid;

#[inline]
pub fn part1(input: &str) -> usize {
    solve(input, find_anti_nodes)
}

#[inline]
pub fn part2(input: &str) -> usize {
    solve(input, find_resonant_anti_nodes)
}

fn solve(input: &str, node_finder_fn: impl Fn(&[(i32, i32)], (i32, i32)) -> Vec<(i32, i32)>) -> usize {
    let (antenna_locations, grid_size) = parse(input);

    antenna_locations
        .iter()
        .map(|(_, locations)| node_finder_fn(locations, grid_size))
        .fold(ahash::AHashSet::with_capacity(2000), |mut acc, anti_nodes| {
            anti_nodes.iter().for_each(|node| {
                acc.insert(*node);
            });
            acc
        })
        .len()
}

fn find_anti_nodes(locations: &[(i32, i32)], grid_size: (i32, i32)) -> Vec<(i32, i32)> {
    let mut anti_nodes = Vec::with_capacity(100);
    for x in 0..locations.len() {
        for y in 0..locations.len() {
            if x == y {
                continue;
            }
            let ant_one = (locations[x].0, locations[x].1);
            let ant_two = (locations[y].0, locations[y].1);
            let distance = (ant_one.0 - ant_two.0, ant_one.1 - ant_two.1);

            let anti_node_one = (ant_one.0 + distance.0, ant_one.1 + distance.1);
            if anti_node_one.0 >= 0
                && anti_node_one.0 < grid_size.0
                && anti_node_one.1 >= 0
                && anti_node_one.1 < grid_size.1
            {
                anti_nodes.push(anti_node_one);
            }

            let anti_node_two = (ant_two.0 - distance.0, ant_two.1 - distance.1);
            if anti_node_two.0 >= 0
                && anti_node_two.0 < grid_size.0
                && anti_node_two.1 >= 0
                && anti_node_two.1 < grid_size.1
            {
                anti_nodes.push(anti_node_two);
            }
        }
    }
    anti_nodes
}

fn find_resonant_anti_nodes(locations: &[(i32, i32)], grid_size: (i32, i32)) -> Vec<(i32, i32)> {
    let mut anti_nodes = Vec::with_capacity(100);
    for x in 0..locations.len() {
        for y in 0..locations.len() {
            if x == y {
                continue;
            }
            let ant_one = (locations[x].0, locations[x].1);
            let ant_two = (locations[y].0, locations[y].1);
            let distance = (ant_one.0 - ant_two.0, ant_one.1 - ant_two.1);

            for i in 0.. {
                let anti_node = (ant_one.0 + i * distance.0, ant_one.1 + i * distance.1);
                if anti_node.0 < 0
                    || anti_node.0 >= grid_size.0
                    || anti_node.1 < 0
                    || anti_node.1 >= grid_size.1
                {
                    break;
                }
                anti_nodes.push(anti_node);
            }

            for i in 0.. {
                let anti_node = (ant_two.0 - i * distance.0, ant_two.1 - i * distance.1);
                if anti_node.0 < 0
                    || anti_node.0 >= grid_size.0
                    || anti_node.1 < 0
                    || anti_node.1 >= grid_size.1
                {
                    break;
                }
                anti_nodes.push(anti_node);
            }
        }
    }
    anti_nodes
}

fn parse(input: &str) -> (AHashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let grid = Grid::new(input);

    let mut antenna_locations = AHashMap::with_capacity(62);

    for i in 0..grid.rows {
        for j in 0..grid.columns {
            let c = grid.get((i, j)).unwrap();
            if c != '.' {
                antenna_locations
                    .entry(c)
                    .or_insert_with(|| Vec::with_capacity(10))
                    .push((i as i32, j as i32));
            }
        }
    }

    (antenna_locations, (grid.rows as i32, grid.columns as i32))
}

crate::aoctest!(14, 320, 34, 1157);
