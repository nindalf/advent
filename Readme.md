# Advent of Code

My solutions, written in Rust. I've tried to write idiomatic, readable code. I generally use iterators where I can.

I attempt to make my code fast, but if it's a choice between fast and readable, I choose readable. No `unsafe` code or SIMD here.

## Benchmarks

Successfully completed problems with the time taken to execute them on my M1 Pro.

### 2024

| Day  | Problem     | Solution    | Part 1 (ms) | Part 2 (ms) | Total (ms) |
|------|-------------|-------------|-------------|-------------|------------|
| 1 | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [Solution](/y2024/src/day1/mod.rs) | 0.04ms | 0.04ms | 0.08ms |
| 2 | [Red-Nosed Reports](https://adventofcode.com/2024/day/2) | [Solution](/y2024/src/day2/mod.rs) | 0.10ms | 0.10ms | 0.19ms |
| 3 | [Mull It Over](https://adventofcode.com/2024/day/3) | [Solution](/y2024/src/day3/mod.rs) | 0.21ms | 0.22ms | 0.43ms |
| 4 | [Ceres Search](https://adventofcode.com/2024/day/4) | [Solution](/y2024/src/day4/mod.rs) | 0.14ms | 0.08ms | 0.22ms |
| 5 | [Print Queue](https://adventofcode.com/2024/day/5) | [Solution](/y2024/src/day5/mod.rs) | 0.13ms | 0.14ms | 0.27ms |
| 6 | [Guard Gallivant](https://adventofcode.com/2024/day/6) | [Solution](/y2024/src/day6/mod.rs) | 0.15ms | 26.91ms | 27.06ms |
| 7 | [Bridge Repair](https://adventofcode.com/2024/day/7) | [Solution](/y2024/src/day7/mod.rs) | 0.17ms | 4.29ms | 4.45ms |
| 8 | [Resonant Collinearity](https://adventofcode.com/2024/day/8) | [Solution](/y2024/src/day8/mod.rs) | 0.02ms | 0.05ms | 0.06ms |
| 9 | [Disk Fragmenter](https://adventofcode.com/2024/day/9) | [Solution](/y2024/src/day9/mod.rs) | 0.25ms | 27.70ms | 27.96ms |
| 10 | [Hoof It](https://adventofcode.com/2024/day/10) | [Solution](/y2024/src/day10/mod.rs) | 0.08ms | 0.05ms | 0.13ms |
| 11 | [Plutonian Pebbles](https://adventofcode.com/2024/day/11) | [Solution](/y2024/src/day11/mod.rs) | 0.04ms | 2.41ms | 2.45ms |
| 12 | [Garden Groups](https://adventofcode.com/2024/day/12) | [Solution](/y2024/src/day12/mod.rs) | 1.07ms | 1.85ms | 2.92ms |
| 13 | [Claw Contraption](https://adventofcode.com/2024/day/13) | [Solution](/y2024/src/day13/mod.rs) | 0.03ms | 0.03ms | 0.07ms |
| 14 | [Restroom Redoubt](https://adventofcode.com/2024/day/14) | [Solution](/y2024/src/day14/mod.rs) | 0.28ms | 30.37ms | 30.65ms |
| 15 | [Warehouse Woes](https://adventofcode.com/2024/day/15) | [Solution](/y2024/src/day15/mod.rs) | 0.42ms | 0.57ms | 0.98ms |
| 16 | [Reindeer Maze](https://adventofcode.com/2024/day/16) | [Solution](/y2024/src/day16/mod.rs) | 2.90ms | 5.83ms | 8.72ms |
| 17 | [Chronospatial Computer](https://adventofcode.com/2024/day/17) | [Solution](/y2024/src/day17/mod.rs) | 0.00ms | 21.80ms | 21.80ms |
| 18 | [RAM Run](https://adventofcode.com/2024/day/18) | [Solution](/y2024/src/day18/mod.rs) | 0.46ms | 2.39ms | 2.84ms |
| 19 | [Linen Layout](https://adventofcode.com/2024/day/19) | [Solution](/y2024/src/day19/mod.rs) | 0.33ms | 0.34ms | 0.66ms |
| 20 | [Race Condition](https://adventofcode.com/2024/day/20) | [Solution](/y2024/src/day20/mod.rs) | 0.65ms | 37.94ms | 38.60ms |
| 21 | [Keypad Conundrum](https://adventofcode.com/2024/day/21) | [Solution](/y2024/src/day21/mod.rs) | 0.00ms | 0.06ms | 0.07ms |
| 22 | [Monkey Market](https://adventofcode.com/2024/day/22) | [Solution](/y2024/src/day22/mod.rs) | 1.12ms | 31.87ms | 32.99ms |
| 23 | [LAN Party](https://adventofcode.com/2024/day/23) | [Solution](/y2024/src/day23/mod.rs) | 0.37ms | 44.13ms | 44.50ms |
| 24 | [Crossed Wires](https://adventofcode.com/2024/day/24) | [Solution](/y2024/src/day24/mod.rs) | 0.03ms | 0.04ms | 0.07ms |
| 25 | [Code Chronicle](https://adventofcode.com/2024/day/25) | [Solution](/y2024/src/day25/mod.rs) | 0.18ms | 0.00ms | 0.18ms |
|  |  | Total | 9.18ms | 239.19ms | 248.37ms |


## Setup

1. Clone this repo - `gh repo clone nindalf/advent` or `git clone git@github.com:nindalf/advent.git`.
2. Install [Rust](https://www.rust-lang.org/learn/get-started) or update - `rustup update`.
3. Install [just](https://just.systems), the command runner.
4. Install [aocgen](https://github.com/nindalf/aocgen), which fetches the problem input and creates the empty files.

## How to use

Run `just` for all the available commands.

By default `just` will run these for the latest year, set by the env variable `AOC_YEAR`.

```
just fetch 15         # fetches the 15th day's problem and input.

just test 15 1_t      # runs day15::tests::part1_test
just test 15 1_r      # runs day15::tests::part1_real
just test 15 1        # runs both tests for day 15 part 1
just test 15 2        # runs both tests for day 15 part 2
just test 15          # runs all 4 tests for day 15

just submit 15 1 1024 # Submit "1024" as the solution for Day 15 Part 1
just submit 15 2 2048 # Submit "2048" as the solution for Day 15 Part 2

just bench 15         # benchmarks day 15 parts 1 and 2
```

`just test`/`just bench` with no arguments runs all the tests/benchmarks for the latest year.

### Overriding `AOC_YEAR`

If `AOC_YEAR` is not set, it picks up the default from the `justfile`. To run the commands for a different year, you can choose one of these options:

- Set it permanently
  - Set the env variable - `export AOC_YEAR=2023`
  - Change the default in the `justfile` - `AOC_YEAR := env_var_or_default("AOC_YEAR", "2023")`
- Set it for one invocation
  - `AOC_YEAR=2023 just test` OR
  - `just --set AOC_YEAR 2023 test`
