use ahash::AHashSet;

#[inline]
pub fn part1(input: &str) -> usize {
    let (towels, patterns) = parse(input);
    patterns
        .iter()
        .filter(|pattern| match_towels_to_pattern(&towels, pattern))
        .count()
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    0
}

fn match_towels_to_pattern(towels: &AHashSet<&str>, pattern: &str) -> bool {
    let mut current = AHashSet::new();
    current.insert(0);
    loop {
        let mut temp = AHashSet::with_capacity(current.len() * 2);
        for index in current {
            for idx in 0..8 {
                let new_len = index + idx + 1;
                if new_len <= pattern.len() && towels.contains(&pattern[index..new_len]) {
                    if new_len == pattern.len() {
                        // Match found
                        return true;
                    }
                    if new_len > pattern.len() {
                        continue;
                    }
                    temp.insert(new_len);
                }
            }
        }
        if temp.is_empty() {
            // No matches found
            return false;
        }
        current = temp;
    }
}

fn parse(input: &str) -> (AHashSet<&str>, Vec<&str>) {
    let (towels, patterns) = input.split_once("\n\n").expect("input is well formed");
    let patterns = patterns.lines().collect();
    let towels = towels.split(", ").collect();
    (towels, patterns)
}

common::aoctest!(6, 236, 1234, 1234);
