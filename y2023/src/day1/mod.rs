use phf::{Map, phf_map};

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

#[inline]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let first = line.chars().find(|c| c.is_numeric())?;
            let last = line.chars().rev().find(|c| c.is_numeric())?;
            let combined = format!("{first}{last}");
            combined.parse::<u32>().ok()
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
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
            let combined = format!("{first}{last}");
            combined.parse::<u32>().ok()
        })
        .sum()
}

pub fn parse(_input: &str) {}

common::aoctest!(209, 54877, 281, 54100);
