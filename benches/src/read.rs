use std::collections::BTreeMap;
use std::path::Path;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use scan_fmt::scan_fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub year: u32,
    pub day: u32,
    pub part_one_millis: f64,
    pub part_two_millis: f64,
    pub total: f64,
}

// Read the benches in the target folder and the csv file, if any.
// If a record exists in the csv file but not in the benches, add it.
pub fn read_all_data(csv_file: &Path) -> Result<BTreeMap<(u32, u32), Record>> {
    let mut benches = read_benches()?;
    let csv_records = read_csv(csv_file)?;
    for record in csv_records {
        benches.entry((record.year, record.day)).or_insert(record);
    }
    Ok(benches)
}

fn read_csv(csv_file: &Path) -> Result<Vec<Record>> {
    if !csv_file.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(csv_file)?;
    let reader: BufReader<File> = BufReader::new(file);
    csv::Reader::from_reader(reader)
        .deserialize()
        .collect::<Result<Vec<Record>, csv::Error>>()
        .map_err(|_| anyhow::anyhow!("Failed to read CSV"))
}

fn read_benches() -> Result<BTreeMap<(u32, u32), Record>> {
    let pattern = "target/criterion/*/new/estimates.json";
    let benches = glob::glob(pattern)?
        .filter_map(|entry| entry.ok())
        .filter_map(|path: std::path::PathBuf| {
            let benchmark_name = path.parent()?.parent()?.file_name()?.to_str()?.to_string();
            let (year, day, part) =
                scan_fmt!(&benchmark_name, "y{d} day{d} Part {d}", u32, u32, u32).ok()?;
            let file = File::open(path).ok()?;
            let bench: Bench = serde_json::from_reader(BufReader::new(file)).ok()?;
            Some((year, day, part, bench.mean.point_estimate))
        })
        .fold(BTreeMap::new(), |mut acc, (year, day, part, time)| {
            let record = acc.entry((year, day)).or_insert(Record {
                year,
                day,
                part_one_millis: 0.0,
                part_two_millis: 0.0,
                total: 0.0,
            });

            match part {
                1 => record.part_one_millis = time / 1000000.0,
                2 => record.part_two_millis = time / 1000000.0,
                _ => {}
            }
            record.total = record.part_one_millis + record.part_two_millis;
            acc
        });

    Ok(benches)
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Bench {
    pub mean: Measurement,
    pub median: Measurement,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Measurement {
    confidence_interval: ConfidenceInterval,
    pub point_estimate: f64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}
