use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::Deserialize;

pub fn read_all_benches() -> Result<Vec<(String, Bench)>> {
    let pattern = "target/criterion/*/new/estimates.json";
    let x = glob::glob(pattern)?
        .filter_map(|entry| entry.ok())
        .filter_map(|path: std::path::PathBuf| {
            let benchmark_name = path.parent()?.parent()?.file_name()?.to_str()?.to_string();
            let file = File::open(path).ok()?;
            let bench: Bench = serde_json::from_reader(BufReader::new(file)).ok()?;
            Some((benchmark_name, bench))
        })
        .collect();
    Ok(x)
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
