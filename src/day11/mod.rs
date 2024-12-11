use ahash::AHashMap;

#[inline]
pub fn part1(input: &str) -> u64 {
    let mut memoized = AHashMap::with_capacity(100000);
    parse(input).map(|n| transform(n, 25, &mut memoized)).sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let mut memoized = AHashMap::with_capacity(100000);
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

crate::aoctest!(55312, 186203, 65601038650482, 221291560078593);
