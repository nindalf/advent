use ahash::{AHashMap, AHashSet};

/// Shoutout to Wikipedia (https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm).
/// First time I've heard of this problem in graph theory. Never even knew heard of "cliques" in computer science.
/// Part 1 is fine - 379µs
/// Part 2 is absurdly slow - 60 milliseconds. I can easily do better.
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
pub fn part2(input: &str) -> String {
    let lan = parse(input);
    let r = AHashSet::new();
    let p = lan.keys().copied().collect();
    let x = AHashSet::new();
    let maximal_clique = bron_kerbosch_clique_calculator(&lan, r, p, x);

    let mut clique_vec: Vec<_> = maximal_clique.iter().copied().collect();
    clique_vec.sort_unstable();
    clique_vec.join(",")
}

fn bron_kerbosch_clique_calculator<'a>(
    lan: &AHashMap<&str, AHashSet<&'a str>>,
    r: AHashSet<&'a str>,
    p: AHashSet<&'a str>,
    x: AHashSet<&str>,
) -> AHashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    // let (_, most_popular_node) = p.iter().map(|node| (lan[node].len(), node)).max().unwrap();
    // println!("{:?}", most_popular_node);

    let mut p_c = p.clone();
    let mut x_c = x.clone();

    let mut results = Vec::with_capacity(p.len());
    for vertex in p.iter() {
        let mut r_c = r.clone();

        r_c.insert(vertex);
        let neighbours = &lan[vertex];
        let pruned_p = p_c.intersection(neighbours).copied().collect();
        let pruned_x = x_c.intersection(neighbours).copied().collect();

        let maximal_clique = bron_kerbosch_clique_calculator(lan, r_c, pruned_p, pruned_x);
        results.push(maximal_clique);

        p_c.remove(vertex);
        x_c.insert(vertex);
    }

    if results.is_empty() {
        return r;
    }
    results.sort_unstable_by_key(|a| a.len());
    results[results.len() - 1].clone()
}

fn parse(input: &str) -> AHashMap<&str, AHashSet<&str>> {
    input
        .lines()
        .filter_map(|line| line.split_once("-"))
        .fold(AHashMap::new(), |mut acc, pair| {
            acc.entry(pair.0)
                .or_insert_with(|| AHashSet::with_capacity(15))
                .insert(pair.1);
            acc.entry(pair.1)
                .or_insert_with(|| AHashSet::with_capacity(15))
                .insert(pair.0);
            acc
        })
}

common::aoctest!(
    7,
    1368,
    "co,de,ka,ta".to_owned(),
    "dd,ig,il,im,kb,kr,pe,ti,tv,vr,we,xu,zi".to_owned()
);
