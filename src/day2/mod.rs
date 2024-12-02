#[inline]
pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|line| is_monotonic(line))
        .count()
}

#[inline]
pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|line| is_monotonic_safe(line))
        .count()
}

fn is_monotonic(numbers: &[u32]) -> bool {
    if numbers.is_empty() || numbers.len() == 1 {
        return false;
    }
    let mut current = numbers[0];
    let trend_increasing = numbers[0] > numbers[1];
    for n in numbers.iter().skip(1) {
        if current.abs_diff(*n) < 1 || current.abs_diff(*n) > 3 {
            return false;
        }
        if trend_increasing && current < *n {
            return false;
        }
        if !trend_increasing && current > *n {
            return false;
        }
        current = *n;
    }
    true
}

fn is_monotonic_safe(numbers: &[u32]) -> bool {
    if is_monotonic(numbers) {
        return true;
    }
    for i in 0..numbers.len() {
        let mut n = numbers.to_vec();
        n.remove(i);
        if is_monotonic(&n) {
            return true;
        }
    }

    false
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split(" "))
        .map(|parts| {
            parts
                .filter_map(|part| part.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect()
}

crate::aoctest!(2, 591, 4, 621);
