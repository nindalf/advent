use anyhow::Result;
use plotlars::{BarPlot, Orientation, Plot, Rgb, Text};
use polars::prelude::*;
use std::collections::BTreeMap;

use crate::read::Record;

#[allow(dead_code)]
pub fn create_bar_chart(data: &BTreeMap<(u32, u32), Record>, year: u32) -> Result<()> {
    // First, let's convert our data into vectors that we can use to create the DataFrame
    let mut days = Vec::with_capacity(12);
    let mut part_one_times = Vec::with_capacity(12);
    let mut part_two_times = Vec::with_capacity(12);
    let mut totals = Vec::with_capacity(12);

    for record in data.values() {
        if record.year != year {
            continue;
        }
        days.push(record.day);
        part_one_times.push(record.part_one_millis);
        part_two_times.push(record.part_two_millis);
        totals.push(record.total);
    }

    // Create the DataFrame
    let df = DataFrame::new(vec![
        Column::new("day".into(), days),
        Column::new("part_one_millis".into(), part_one_times),
        Column::new("part_two_millis".into(), part_two_times),
        Column::new("total".into(), totals),
    ])?;

    BarPlot::builder()
        .data(&df)
        .labels("day")
        .values("total")
        .orientation(Orientation::Vertical)
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(
            Text::from(format!("Total time taken for each day in {}", year))
                .font("Arial")
                .size(18),
        )
        .x_title(Text::from("Day").font("Arial").size(15))
        .y_title(Text::from("Time (ms)").font("Arial").size(15))
        .build()
        .plot();

    Ok(())
}
