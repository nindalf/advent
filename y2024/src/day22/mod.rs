use ahash::AHashMap;

#[inline]
pub fn part1(input: &str) -> i64 {
    parse(input).map(|seed| random_stream(seed, 2000)).sum()
}

#[inline]
pub fn part2(input: &str) -> i64 {
    let all_sequences =
        parse(input)
            .map(price_sequences)
            .fold(AHashMap::new(), |mut acc, price_sequences| {
                for (seq, price) in price_sequences {
                    *acc.entry(seq).or_default() += price
                }
                acc
            });

    all_sequences.values().max().copied().unwrap()
}

fn random_stream(seed: i64, n: i64) -> i64 {
    let mut intermediate = seed;
    for _ in 0..n {
        intermediate = next_random(intermediate);
    }
    intermediate
}

// Clippy lint wants me to change `i in 0..4`` to `for t in temp.iter_mut()`
// But this obscures the fact that I run that loop 4 times and the other loop 1996 times.
#[allow(clippy::needless_range_loop)]
fn price_sequences(seed: i64) -> AHashMap<(i64, i64, i64, i64), i64> {
    let mut result = AHashMap::new();

    let mut intermediate = seed;
    let mut previous_price = intermediate % 10;
    let mut temp = [0; 4];
    for i in 0..4 {
        intermediate = next_random(intermediate);
        let price = intermediate % 10;
        let change = price - previous_price;
        previous_price = price;
        temp[i] = change;
    }
    let mut sequence = (temp[0], temp[1], temp[2], temp[3]);
    result.insert(sequence, previous_price);

    for _ in 4..2000 {
        intermediate = next_random(intermediate);
        let price = intermediate % 10;
        let change = price - previous_price;
        previous_price = price;
        sequence = (sequence.1, sequence.2, sequence.3, change);
        if !result.contains_key(&sequence) {
            result.insert(sequence, price);
        }
    }
    result
}

#[inline(always)]
fn next_random(mut n: i64) -> i64 {
    n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n) % 16777216;
    n
}

fn parse(input: &str) -> impl Iterator<Item = i64> + use<'_> {
    input.lines().flat_map(str::parse)
}

common::aoctest!(37990510, 18525593556, 23, 2089);
