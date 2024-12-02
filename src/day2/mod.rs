#[inline]
pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|line| is_monotonic(line, None))
        .count()
}

#[inline]
pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|line| is_monotonic_safe(line))
        .count()
}

fn is_monotonic_safe(numbers: &[u32]) -> bool {
    if is_monotonic(numbers, None) {
        return true;
    }

    for i in 0..numbers.len() {
        if is_monotonic(numbers, Some(i)) {
            return true;
        }
    }

    false
}

fn is_monotonic(numbers: &[u32], skip_index: Option<usize>) -> bool {
    if numbers.len() == 0 || numbers.len() == 1 {
        return false;
    }
    let (first, second) = match skip_index {
        Some(0) => (1, 2),
        Some(1) => (0, 2),
        _ => (0, 1),
    };

    let mut current = numbers[first];
    let trend_increasing = numbers[first] > numbers[second];
    let skip_index = match skip_index {
        Some(s) => s,
        None => numbers.len() + 1,
    };
    for i in second..numbers.len() {
        if i == skip_index {
            continue
        }
        let n = numbers[i];
        if current.abs_diff(n) < 1 || current.abs_diff(n) > 3 {
            return false;
        }
        if (trend_increasing && current < n) || (!trend_increasing && current > n) {
            return false;
        }
        current = n;
    }
    return true;
}


fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split(" "))
        .map(|parts| {
            parts
                .flat_map(|part| part.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .collect()
}

crate::aoctest!(2, 591, 4, 621);
