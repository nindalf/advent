use ahash::{AHashMap, AHashSet};

#[inline]
pub fn part1(input: &str) -> usize {
    let lan = parse(input);
    let mut results = AHashSet::with_capacity(10000);
    for (node, connections) in lan.iter() {
        if !node.starts_with("t") {
            continue;
        }
        for connection in connections {
            for other_connection in connections {
                if lan[connection].contains(other_connection) {
                    let mut tmp = [node, connection, other_connection];
                    tmp.sort();
                    results.insert(tmp);
                }
            }
        }
    }
    results.len()
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    0
}

fn parse(input: &str) -> AHashMap<&str, Vec<&str>> {
    input
        .lines()
        .filter_map(|line| line.split_once("-"))
        .fold(AHashMap::new(), |mut acc, pair| {
            acc.entry(pair.0).or_default().push(pair.1);
            acc.entry(pair.1).or_default().push(pair.0);
            acc
        })
}

common::aoctest!(7, 1368, 1234, 1234);
