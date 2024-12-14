# Advent of Code

My solutions, written in Rust. I've tried to write idiomatic, readable code. I generally use iterators where I can.

I attempt to make my code fast, but if it's a choice between fast and readable, I choose readable. No unsafe code or SIMD here.

## Setup

1. Clone this repo - `gh repo clone nindalf/advet` or `git clone git@github.com:nindalf/advent.git`
2. [Install Rust](https://www.rust-lang.org/learn/get-started) or update - `rustup update`
3. Install just - `brew install just`
4. `just test`

## How to use

See the justfile for common commands 

```
# Change the AOC_YEAR in the justfile or `export AOC_YEAR=2025`
# Or set the variable while invoking just - `just --set AOC_YEAR 2023 fetch 1`

just fetch 15 # fetches the 15th day's problem and input.

just test 15 1_t # runs day15::tests::part1_test
test test 15 2   # runs day15::tests::part2_test _and_ day15::tests:part2
just test 15     # runs all 4 tests for day 15
just test        # runs all tests for all days of CURRENT_YEAR

just bench 15    # benchmarks day 15 parts 1 and 2
just bench       # runs all benchmarks for all days of CURRENT_YEAR
```
