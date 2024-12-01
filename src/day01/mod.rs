use ahash::AHashMap;

#[inline]
pub fn part_1(input: &str) -> i32 {
    let (mut first, mut second) = parse(input);
    first.sort_unstable();
    second.sort_unstable();
    first
        .iter()
        .zip(second.iter())
        .map(|(first, second)| (first - second).abs())
        .sum()
}

#[inline]
pub fn part_2(input: &str) -> i32 {
    let (first, second) = parse(input);

    let mut counts = AHashMap::with_capacity(1000);
    for i in second {
        *counts.entry(i).or_insert(0) += 1;
    }
    first.iter().map(|i| i * counts.get(i).unwrap_or(&0)).sum()
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .filter_map(|(first, second)| {
            let first = first.parse::<i32>().ok();
            let second = second.parse::<i32>().ok();
            first.zip(second)
        })
        .unzip()
}

crate::aoctest!(11, 2000468, 31, 18567089);
