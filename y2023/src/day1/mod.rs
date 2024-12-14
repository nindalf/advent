use phf::{phf_map, Map};

const NUMBERS: Map<&str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
    "zero" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "0" => 0,
};

/// Note on performance: The original version written in 2023 ran in 66.372 µs and 196.57 µs
/// I tried a different algo for part 2 that was 4.35x slower. I didn't expect that.
/// In any case I made a small change - multiplication + addition instead of string format + parse.
/// This improved performance to 25.19 µs (-62%) and 159.05 µs (-19%).
#[inline]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let first = line.chars().find(|c| c.is_numeric())?;
            let last = line.chars().rev().find(|c| c.is_numeric())?;
            Some(((first as u8 - b'0') * 10 + (last as u8 - b'0')) as u32)
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u32 {
    input.lines().map(old).sum()
}

fn old(line: &str) -> u32 {
    let mut numbers = Vec::with_capacity(10);
    let mut remaining = line;
    while !remaining.is_empty() {
        for (k, v) in NUMBERS.entries() {
            if remaining.starts_with(k) {
                numbers.push(v);
                break;
            }
        }
        remaining = &remaining[1..];
    }
    let first = numbers[0];
    let last = numbers[numbers.len() - 1];
    first * 10 + last
}

#[allow(dead_code)]
fn new(line: &str) -> u32 {
    let (mut first, mut first_index) = (0, line.len());
    let (mut last, mut last_index) = (0, 0);

    for (k, v) in NUMBERS.entries() {
        let matches: Vec<_> = line.match_indices(k).collect();
        if !matches.is_empty() && matches[0].0 <= first_index {
            first_index = matches[0].0;
            first = *v;
        }
        if !matches.is_empty() && matches[matches.len() - 1].0 >= last_index {
            last_index = matches[matches.len() - 1].0;
            last = *v;
        }
    }
    first * 10 + last
}

pub fn parse(_input: &str) {}

common::aoctest!(209, 54877, 281, 54100);
