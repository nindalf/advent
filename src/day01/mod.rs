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
    let mut counts = std::collections::HashMap::new();
    for i in second {
        *counts.entry(i).or_insert(0) += 1;
    }
    first.iter().map(|i| i * counts.get(i).unwrap_or(&0)).sum()
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(first, second)| {
            let first = first.parse::<i32>().unwrap();
            let second = second.parse::<i32>().unwrap();
            (first, second)
        })
        .unzip()
}

crate::aoctest!(11, 2000468, 31, 18567089);
