use ahash::{AHashMap, AHashSet};

/// Shoutout to Wikipedia (https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm).
/// First time I've heard of this problem in graph theory. Never even knew heard of "cliques" in computer science.
/// Part 1 is fine - 379µs
/// Part 2 is absurdly slow - 60 milliseconds. I can easily do better.
///
/// Performance
/// 1. Remove set X. In this case it didn't seem to do anythin. Part 2 - 44.1ms (-54.1%).
/// 2. I should be able to use the more performant version of Bron-Kerbosch but I wasn't able to make it work.
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
    let maximal_clique = bron_kerbosch_pivoted(&lan);
    let mut clique_vec: Vec<_> = maximal_clique.iter().copied().collect();
    clique_vec.sort_unstable();
    clique_vec.join(",")
}

fn bron_kerbosch_pivoted<'a>(lan: &'a AHashMap<&str, AHashSet<&'a str>>) -> AHashSet<&'a str> {
    let r = AHashSet::new();
    let p: AHashSet<_> = lan.keys().copied().collect();

    if p.is_empty() {
        return r;
    }

    // Choose pivot vertex with most neighbors in P
    let pivot = p
        .iter()
        .max_by_key(|&v| lan[v].intersection(&p).count())
        .unwrap();

    let mut p_minus_n = p.difference(&lan[pivot]).copied().collect::<AHashSet<_>>();
    let mut results = Vec::with_capacity(p_minus_n.len());

    while !p_minus_n.is_empty() {
        let vertex = p_minus_n.iter().next().unwrap().to_owned();
        p_minus_n.remove(&vertex);

        let mut r_new = r.clone();
        r_new.insert(vertex);

        let neighbors = &lan[vertex];
        let p_new = p.intersection(neighbors).copied().collect();

        let clique = bron_kerbosch_pivoted_recursive(lan, r_new, p_new);
        results.push(clique);
    }

    results.into_iter().max_by_key(|c| c.len()).unwrap_or(r)
}

fn bron_kerbosch_pivoted_recursive<'a>(
    lan: &AHashMap<&str, AHashSet<&'a str>>,
    r: AHashSet<&'a str>,
    p: AHashSet<&'a str>,
) -> AHashSet<&'a str> {
    if p.is_empty() {
        return r;
    }

    let pivot = p
        .iter()
        .max_by_key(|&v| lan[v].intersection(&p).count())
        .unwrap();

    let mut results = Vec::new();
    for vertex in p.difference(&lan[pivot]) {
        let mut r_new = r.clone();
        r_new.insert(vertex);

        let p_new = p.intersection(&lan[vertex]).copied().collect();
        results.push(bron_kerbosch_pivoted_recursive(lan, r_new, p_new));
    }

    results.into_iter().max_by_key(|c| c.len()).unwrap_or(r)
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
