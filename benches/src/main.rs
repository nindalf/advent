use std::collections::{BTreeMap, HashMap};

use read::Bench;
use scan_fmt::scan_fmt;
use write::Record;

mod read;
mod write;

fn transform(raw: &[(String, Bench)]) -> BTreeMap<(u32, u32), Record> {
    let mut intermediate = HashMap::new();
    for (name, bench) in raw {
        let (year, day, part) = match scan_fmt!(&name, "y{d} day{d} Part {d}", u32, u32, u32) {
            Ok(x) => x,
            Err(_) => continue,
        };
        intermediate.insert((year, day, part), bench.mean.point_estimate);
    }

    let mut results = BTreeMap::new();
    for ((year, day, part), time) in intermediate {
        let record = results.entry((year, day)).or_insert(Record {
            part_one_millis: 0.0,
            part_two_millis: 0.0,
            total: 0.0,
        });

        match part {
            1 => record.part_one_millis = time / 1000000.0,
            2 => record.part_two_millis = time / 1000000.0,
            _ => continue,
        }
        record.total = record.part_one_millis + record.part_two_millis;
    }
    results
}

fn main() -> anyhow::Result<()> {
    // Read all files that match the glob patter /target/criterion/*/new/estimates.json
    let raw = read::read_all_benches()?;
    let transformed = transform(&raw);
    write::write_to_readme(&transformed)?;

    Ok(())
}
