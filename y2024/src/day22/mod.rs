#[inline]
pub fn part1(input: &str) -> u64 {
    parse(input).map(|seed| random_stream(seed, 2000)).sum()
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    0
}

fn random_stream(seed: u64, n: u64) -> u64 {
    let mut intermediate = seed;
    for _ in 0..n {
        intermediate = ((intermediate * 64) ^ intermediate) % 16777216;
        intermediate = ((intermediate / 32) ^ intermediate) % 16777216;
        intermediate = ((intermediate * 2048) ^ intermediate) % 16777216;
    }
    intermediate
}

fn parse(input: &str) -> impl Iterator<Item = u64> + use<'_> {
    input.lines().flat_map(str::parse)
}

common::aoctest!(37327623, 18525593556, 1234, 1234);
