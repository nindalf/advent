use ahash::AHashMap;

/// Performance: The first version of this ran in 187.59 µs and 4.43ms.
/// There was only one optimisation I could think of - size the HashMap to the size actually needed by the inputs.
/// The final sizes in both parts were 3.3k and 125k. I changed 100k in both cases to 10k (minimising allocation)
/// and 150k (minimising re-allocation).
/// Final time: 45.87 µs (-75%) and 2.84ms (-35%)
#[inline]
pub fn part1(input: &str) -> u64 {
    let mut memoized = AHashMap::with_capacity(10000);
    parse(input).map(|n| transform(n, 25, &mut memoized)).sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let mut memoized = AHashMap::with_capacity(150000);
    parse(input).map(|n| transform(n, 75, &mut memoized)).sum()
}

fn transform(n: u64, remaining_generations: u64, memoized: &mut AHashMap<(u64, u64), u64>) -> u64 {
    // Leaf, no more generations
    if remaining_generations == 0 {
        return 1;
    }
    if let Some(answer) = memoized.get(&(n, remaining_generations)) {
        return *answer;
    }
    let progeny = match n {
        n if n == 0 => transform(1, remaining_generations - 1, memoized),
        n if (n.ilog10() % 2 == 1) => {
            let (first, second) = split(n);
            transform(first, remaining_generations - 1, memoized)
                + transform(second, remaining_generations - 1, memoized)
        }
        n @ _ => transform(n * 2024, remaining_generations - 1, memoized),
    };
    memoized.insert((n, remaining_generations), progeny);
    progeny
}

fn split(n: u64) -> (u64, u64) {
    let operand = 10u64.pow(n.ilog10() / 2 + 1);
    (n / operand, n % operand)
}

fn parse(input: &str) -> impl Iterator<Item = u64> + use<'_> {
    input.split_ascii_whitespace().flat_map(str::parse)
}

common::aoctest!(55312, 186203, 65601038650482, 221291560078593);
