use ahash::{AHashMap, AHashSet};
use rayon::prelude::*;

/// Note on performance
/// I started out minimising string allocations, using only indices instead of allocating strings, or storing strings.
/// In fact, there is no string allocation, not even the input, which is a static string baked into the binary. Everything borrows from that.
/// I started with the towels in a Vec, checking each time if the string starting at index matched each of the towels.
/// That was slow enough that it never actually finished part 1.
/// I replaced that with towels being a HashSet and checking the prefix of pattern is contained in the HashSet. Much faster
/// Part 1 completed in 8.2ms single threaded and 1.2ms (-85%) while using rayon.
/// I had to rewrite it with a recursive approach to make it work for part 2. This further optimised performance to 467µs (-61%).
///
/// Final numbers: Part 1 - 451µs, Part 2 - 454µs.
/// They're both doing exactly the same computation but Part 2 takes slightly longer because it needs to add up the results.
/// I could optimise Part 1 with a const generic param so it returns after just one match, but I prefer to keep the code looking simple and clean.
///
/// One more attempted optimisation - using a shared multi-threaded cache among all runs instead of a separate single-threaded cache for each one.
/// The shared `&dashmap::DashMap` regresses performance by 130-140%. I guess for this input the amount of overlap between days
/// isn't enough to justify the overhead of a multi-threaded map.
#[inline]
pub fn part1(input: &str) -> usize {
    let (towels, patterns) = parse(input);
    patterns
        .iter()
        .par_bridge()
        .filter(|pattern| {
            match_towels_to_pattern(&towels, pattern, &mut AHashMap::with_capacity(100)) > 0
        })
        .count()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let (towels, patterns) = parse(input);
    patterns
        .iter()
        .par_bridge()
        .map(|pattern| match_towels_to_pattern(&towels, pattern, &mut AHashMap::with_capacity(100)))
        .sum()
}

fn match_towels_to_pattern<'a>(
    towels: &AHashSet<&str>,
    pattern: &'a str,
    cache: &mut AHashMap<&'a str, u64>,
) -> u64 {
    if let Some(pre_computed) = cache.get(pattern) {
        return *pre_computed;
    }

    let mut matches = 0;
    for idx in 1..=8 {
        if idx <= pattern.len() && towels.contains(&pattern[0..idx]) {
            if idx == pattern.len() {
                matches += 1;
                break;
            }
            matches += match_towels_to_pattern(towels, &pattern[idx..], cache);
        }
    }

    *cache.entry(pattern).or_default() += matches;

    matches
}

fn parse(input: &str) -> (AHashSet<&str>, Vec<&str>) {
    let (towels, patterns) = input.split_once("\n\n").expect("input is well formed");
    let patterns = patterns.lines().collect();
    let towels = towels.split(", ").collect();
    (towels, patterns)
}

common::aoctest!(6, 236, 16, 643685981770598);
