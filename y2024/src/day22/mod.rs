use ahash::AHashMap;
use rayon::prelude::*;

/// Performance
/// Initial run time: 24.336 ms and 350ms.
/// 1. Rayon: Reduces this to 3.12ms (-87%) and 76ms (-78%).
/// 2. Replaced multiplication and modulo with bitwise operations: 1.1ms (-64%) and 76ms (-7.7%).
///    Bit disappointed with LLVM here. 😅
/// 3. Replaced a HashMap check and insert with an entry-insert. 1.1ms (no change) and 72ms (-5%).
/// 4. Used HashMap::with_capacity instead of HashMap::new to reduce reallocations.
///    Size chosen is the max possible - 19C4 - 3876. 1.1ms (no change) and 61ms (-14%).
/// 5. Replace HashMap key (i64,i64,i64,i64) with a single i64 made by multiplying them with primes + summing.
///    1.1ms (no change) and 31.8ms (-49%).
#[inline]
pub fn part1(input: &str) -> i64 {
    parse(input).map(random_stream_2000).sum()
}

#[inline]
pub fn part2(input: &str) -> i64 {
    let all_sequences: AHashMap<i64, i64> = parse(input).map(price_sequences).reduce(
        || AHashMap::with_capacity(4000),
        |mut acc, price_sequences| {
            for (seq, price) in price_sequences {
                *acc.entry(seq).or_default() += price
            }
            acc
        },
    );

    all_sequences.values().max().copied().unwrap()
}

fn random_stream_2000(seed: i64) -> i64 {
    let mut intermediate = seed;
    for _ in 0..2000 {
        intermediate = next_random(intermediate);
    }
    intermediate
}

// Clippy lint wants me to change `i in 0..4`` to `for t in temp.iter_mut()`
// But this obscures the fact that I run that loop 4 times and the other loop 1996 times.
#[allow(clippy::needless_range_loop)]
fn price_sequences(seed: i64) -> AHashMap<i64, i64> {
    let mut result = AHashMap::with_capacity(4000);

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
    result.insert(index(sequence), previous_price);

    for _ in 4..2000 {
        intermediate = next_random(intermediate);
        let price = intermediate % 10;
        let change = price - previous_price;
        previous_price = price;
        sequence = (sequence.1, sequence.2, sequence.3, change);
        result.entry(index(sequence)).or_insert(price);
    }
    result
}

#[inline(always)]
fn next_random(mut n: i64) -> i64 {
    n = ((n << 6) ^ n) & 0xffffff;
    n = ((n >> 5) ^ n) & 0xffffff;
    ((n << 11) ^ n) & 0xffffff
}

#[inline(always)]
fn index(seq: (i64, i64, i64, i64)) -> i64 {
    6859 * (seq.0 + 10) + 361 * (seq.1 + 10) + 19 * (seq.2 + 10) + (seq.3 + 10)
}

fn parse(input: &str) -> impl ParallelIterator<Item = i64> + use<'_> {
    input.par_lines().flat_map(str::parse)
}

common::aoctest!(37990510, 18525593556, 23, 2089);
