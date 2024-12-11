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

            // criterion_group!(
            //     [<$year _benches>],
            //     $([<$year _ $day>],)*
            // );
        }
    }
}

macro_rules! benchmarks {

    ($year:ident, $($day:ident),+) => {
        paste!{
        $(
            benchmark_year!{$year, $day}
        )+

        criterion_group!(benches, $([<$year _ $day>],)+);
        criterion_main!(benches);
    }
    }
}

benchmarks! {y2024, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11}
