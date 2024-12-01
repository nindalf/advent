use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! benchmark {
    ($name: ident) => {
        paste::paste!{
            static [<$name:upper _INPUT>]: &str = include_str!(concat!("../src/", stringify!($name), "/input.txt"));

            fn $name(c: &mut Criterion) {
                c.bench_function(concat!(stringify!($name), " Part 1"), |b| {
                    b.iter(|| advent_2024::$name::part_1(black_box([<$name:upper _INPUT>])));
                });
                c.bench_function(concat!(stringify!($name), " Part 2"), |b| {
                    b.iter(|| advent_2024::$name::part_2(black_box([<$name:upper _INPUT>])));
                });
            }
        }
    };
}

macro_rules! benchmarks {
    ($($name:ident),+) => {
        $(
            benchmark!{$name}
        )+

        criterion_group!(benches, $($name,)+);
        criterion_main!(benches);
    }
}

benchmarks! {day1}
