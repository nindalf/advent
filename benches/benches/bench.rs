use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

// Macro to generate benchmarks for a specific year
macro_rules! benchmark_year {
    ($year:ident, $($day:ident),+) => {
        paste! {
            $(
                static [<$year:upper _ $day:upper _INPUT>]: &str = include_str!(
                    concat!("../../", stringify!($year), "/src/", stringify!($day), "/input.txt")
                );

                fn [<$year _ $day>](c: &mut Criterion) {
                    c.bench_function(
                        &format!("{} {} Part 1", stringify!($year), stringify!($day)),
                        |b| {
                            b.iter(||
                                $year::$day::part1(black_box([<$year:upper _ $day:upper _INPUT>]))
                            );
                        }
                    );
                    c.bench_function(
                        &format!("{} {} Part 2", stringify!($year), stringify!($day)),
                        |b| {
                            b.iter(||
                                $year::$day::part2(black_box([<$year:upper _ $day:upper _INPUT>]))
                            );
                        }
                    );
                }
            )+
        }
    }
}

macro_rules! benchmarks {
    // Match one or more year-days groups
    ($($year:ident {$($day:ident),+ $(,)?}),+ $(,)?) => {
        paste! {
            // For each year-day combination, generate the benchmark
            $(
                $(
                    benchmark_year!{$year, $day}
                )+
            )+

            // Create a single criterion group with all benchmarks
            criterion_group!(
                benches,
                $(
                    $(
                        [<$year _ $day>],
                    )+
                )+
            );
            criterion_main!(benches);
        }
    };
}

benchmarks! {
    y2023 {day1, day2},
    y2024 {
        day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
        day14, day15, day16, day17, day18, day19, day20, day22, day23
    },
}
