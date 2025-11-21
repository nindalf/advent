use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[crabtime::function]
fn gen_benches() {
    // Find all solution modules
    // Search the project root for crates that match the pattern y20*/src/day*/mod.rs
    // Return a list of tuples containing the year and day of each solution module
    let pattern = format!(
        "{}/{}",
        crabtime::WORKSPACE_PATH, "y20*/src/day*/mod.rs"
    );
    let mut years_and_days = glob::glob(&pattern)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|path| {
            let components = path
                .components()
                .map(|c| c.as_os_str())
                .filter_map(|c| c.to_str())
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            let year = components[components.len() - 4].clone();
            let day = components[components.len() - 2].clone();
            Some((year, day))
        })
        .collect::<Vec<(String, String)>>();

    // Sort the list so "day2" appears before "day10"
    years_and_days.sort_by(|a, b| match a.1.len().cmp(&b.1.len()) {
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        ordering => ordering,
    });

    // For every year and day ("y2024", "day3"), generate a benchmark function
    // Store the benchmark function names in a vector and pass them to the criterion_group macro
    let mut fn_names = Vec::new();
    for (year, day) in years_and_days {
        let input_file_path = &format!("{}/{}/src/{}/input.txt", crabtime::WORKSPACE_PATH, year, day);
        let input_file = format!("{}_{}_INPUT", year, day);

        let fn_name = format!("{}_{}", year, day);
        fn_names.push(fn_name.clone());

        let module_name = format!("{}::{}", year, day);

        let test_str_one = format!("{} {} Part 1", year, day);
        let test_str_two = format!("{} {} Part 2", year, day);

        crabtime::output! {
            static {{input_file}}: &str = include_str!(stringify!({{input_file_path}}));
            fn {{fn_name}}(c: &mut Criterion) {
                c.bench_function(
                    stringify!({{test_str_one}}),
                    |b| {
                        b.iter(||
                            {{module_name}}::part1(black_box({{input_file}}))
                        );
                    }
                );
                c.bench_function(
                    stringify!({{test_str_two}}),
                    |b| {
                        b.iter(||
                            {{module_name}}::part2(black_box({{input_file}}))
                        );
                    }
                );
            }
        }
    }
    let fn_names = fn_names.join(", ");
    crabtime::output! {
        criterion_group!(benches, {{fn_names}});
        criterion_main!(benches);
    }
}

// I don't know why rust-analyzer says "Compilation of the generated code failed."
// rustc compiles it.
gen_benches!();
