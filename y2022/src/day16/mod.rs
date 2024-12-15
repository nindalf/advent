use ahash::AHashMap;

// Shoutout to /u/zopatista and others from https://redd.it/zo21au
#[inline]
pub fn part1(input: &str) -> u32 {
    let valves = parse(input);
    let distances = min_distances_floyd_warshall(&valves);
    let important_valves: Vec<_> = valves
        .iter()
        .filter(|v| v.steam > 0)
        .map(|v| v.idx)
        .collect();

    find_max_flow_2(
        &valves,
        0,
        &important_valves,
        Vec::new(),
        &distances,
        0,
        0,
        30,
    );
    0
}

#[inline]
pub fn part2(_input: &str) -> u32 {
    0
}

// https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm
#[allow(clippy::needless_range_loop)]
fn min_distances_floyd_warshall(valves: &[Valve]) -> Vec<Vec<u32>> {
    let n = valves.len();
    let mut distances = vec![vec![0; valves.len()]; valves.len()];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                distances[i][j] = 0;
            } else {
                distances[i][j] = 1000;
            }
        }
    }
    for valve in valves.iter() {
        for edge in valve.edges.iter() {
            distances[valve.idx][*edge] = 1;
            distances[*edge][valve.idx] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances[i][j] > distances[i][k] + distances[j][k] {
                    distances[i][j] = distances[i][k] + distances[j][k];
                    distances[j][i] = distances[i][k] + distances[j][k];
                }
            }
        }
    }

    distances
}

#[allow(clippy::too_many_arguments)]
fn find_max_flow_2(
    valves: &[Valve],
    current_valve: usize,
    important_valves: &[usize],
    opened_so_far: Vec<usize>,
    distances: &Vec<Vec<u32>>,
    steam_so_far: u32,
    steam_rate: u32,
    remaining_time: u32,
) -> u32 {
    dbg!(&opened_so_far);
    if remaining_time == 0 {
        return steam_so_far;
    }
    if remaining_time == 1 {
        // it doesn't matter what you do here
        // whether you travel or open, it won't change the flow rate
        return steam_so_far + steam_rate;
    }
    let valve = &valves[current_valve];
    let mut results = Vec::with_capacity(valve.edge_names.len() + 1);

    // Open the valve
    if !opened_so_far.contains(&current_valve) && remaining_time > 1 {
        let increased_steam_rate = steam_rate + valve.steam; // Opened the valve
        for destination in important_valves {
            if opened_so_far.contains(destination) {
                continue;
            }
            let time_to_destination = distances[current_valve][*destination];
            let new_remaining_time = remaining_time - 1 - time_to_destination; // Spent 1 minute opening the valve
            let new_steam_so_far =
                steam_so_far + steam_rate + time_to_destination * increased_steam_rate;
            let mut new_opened_so_far = opened_so_far.clone();
            new_opened_so_far.push(current_valve);

            let max_steam_this_path = find_max_flow_2(
                valves,
                *destination,
                important_valves,
                new_opened_so_far,
                distances,
                new_steam_so_far,
                increased_steam_rate,
                new_remaining_time,
            );
            results.push(max_steam_this_path);
        }
    }

    // // Don't open the valve
    // for destination in important_valves {
    //     if opened_so_far.contains(destination) {
    //         continue;
    //     }
    //     let time_to_destination = distances[current_valve][*destination];
    //     let new_remaining_time = remaining_time - time_to_destination;
    //     let new_steam_so_far = steam_so_far + time_to_destination * steam_rate;
    //     let new_opened_so_far = opened_so_far.clone();

    //     let max_steam_this_path = find_max_flow_2(
    //         valves,
    //         *destination,
    //         important_valves,
    //         new_opened_so_far,
    //         distances,
    //         new_steam_so_far,
    //         steam_rate,
    //         new_remaining_time,
    //     );
    //     results.push(max_steam_this_path);
    // }
    if results.is_empty() {
        return steam_so_far + steam_rate + (steam_rate + valve.steam) * (remaining_time - 1);
    }

    results.sort_unstable();
    results[results.len() - 1]
}

// This doesn't work because of cycles causing exponential run time.
// It is necessary to visit the same node more than once and even the same edge (source destination pair).
// There isn't a good way to make this work. Keeping this here just a curio.
#[allow(dead_code)]
fn find_max_flow(
    valves: &AHashMap<String, Valve>,
    current_valve: &str,
    opened_so_far: Vec<String>,
    travelled_edges: AHashMap<String, u32>,
    steam_so_far: u32,
    steam_rate: u32,
    remaining_time: u32,
) -> u32 {
    if remaining_time == 0 {
        return steam_so_far;
    }
    if remaining_time == 1 {
        // it doesn't matter what you do here
        // whether you travel or open, it won't change the flow rate
        return steam_so_far + steam_rate;
    }
    let valve = &valves[current_valve];
    let mut results = Vec::with_capacity(valve.edge_names.len() + 1);
    // 2 options
    // 1. Spend 1 minute opening this valve if the steam > 0
    // 2. Go straight ahead opening other valves

    if !opened_so_far.contains(&valve.name) && remaining_time > 1 {
        let increased_steam_rate = steam_rate + valve.steam; // Opened the valve
        let new_steam_so_far = steam_so_far + steam_rate + increased_steam_rate;
        let new_remaining_time = remaining_time - 2; // Spent 1 minute opening the valve
        for destination in &valve.edge_names {
            let mut new_opened_so_far = opened_so_far.clone();
            new_opened_so_far.push(valve.name.to_owned());

            let edge = format!("{}-{}", valve.name, destination);
            let mut new_travelled_edges = travelled_edges.clone();
            *new_travelled_edges.entry(edge.clone()).or_default() += 1;
            if new_travelled_edges[&edge] > 2 {
                continue;
            }

            let max_steam_this_path = find_max_flow(
                valves,
                destination,
                new_opened_so_far,
                new_travelled_edges,
                new_steam_so_far,
                increased_steam_rate,
                new_remaining_time,
            );
            results.push(max_steam_this_path);
        }
    }
    if remaining_time > 0 {
        let new_steam_so_far = steam_so_far + steam_rate;
        let new_remaining_time = remaining_time - 1; // Didn't open the valve
        for destination in &valve.edge_names {
            let new_opened_so_far = opened_so_far.clone();

            let edge = format!("{}-{}", valve.name, destination);
            let mut new_travelled_edges = travelled_edges.clone();
            *new_travelled_edges.entry(edge.clone()).or_default() += 1;
            if new_travelled_edges[&edge] > 2 {
                continue;
            }
            let max_steam_this_path = find_max_flow(
                valves,
                destination,
                new_opened_so_far,
                new_travelled_edges,
                new_steam_so_far,
                steam_rate,
                new_remaining_time,
            );
            results.push(max_steam_this_path);
        }
    }

    if results.is_empty() {
        return steam_so_far + steam_rate + (steam_rate + valve.steam) * (remaining_time - 1);
    }

    results.sort_unstable();
    results[results.len() - 1]
}

#[derive(Debug, Default)]
struct Valve {
    idx: usize,
    name: String,
    steam: u32,
    edge_names: Vec<String>,
    edges: Vec<usize>,
}

// This is parsing the file
// Then assigning an integer index to each valve to make lookups easier.
// The list of edges is translated to a list of indices
fn parse(input: &str) -> Vec<Valve> {
    let mut valves: Vec<_> = input
        .lines()
        .flat_map(|mut line| parse_valve(&mut line))
        .collect();

    let mut lookup = AHashMap::with_capacity(valves.len());
    for (idx, valve) in valves.iter_mut().enumerate() {
        valve.idx = idx;
        lookup.insert(valve.name.clone(), idx);
    }
    for valve in valves.iter_mut() {
        let edge_names: Vec<usize> = valve
            .edge_names
            .iter()
            .filter_map(|edge| lookup.get(edge).copied())
            .collect();
        valve.edges = edge_names;
    }

    valves
}

use winnow::ascii::{alpha1, digit1};
use winnow::combinator::{alt, opt, repeat, seq, terminated};
use winnow::token::literal;
use winnow::{PResult, Parser};

fn parse_valve(input: &mut &str) -> PResult<Valve> {
    // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
    seq!{
        Valve {
            _: literal("Valve "),
            name: alpha1.map(|s: &str| s.to_string()),
            _: literal(" has flow rate="),
            steam: digit1.map(|s: &str| s.parse().unwrap()),
            _: alt((literal("; tunnels lead to valves "), literal("; tunnel leads to valve "))),
            edge_names: repeat(1.., terminated(alpha1.map(|s: &str| s.to_string()), opt(literal(", "))))
                .fold(|| Vec::with_capacity(5), |mut acc, valve| {
                    acc.push(valve);
                    acc
                }),
            ..Default::default()
        }
    }.parse_next(input)
}

common::aoctest!(1651, 1234, 1234, 1234);
