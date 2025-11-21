use anyhow::Result;
use plotters::prelude::*;
use std::collections::BTreeMap;

use crate::read::Record;

#[allow(dead_code)]
pub fn create_bar_chart(data: &BTreeMap<(u32, u32), Record>, year: u32) -> Result<()> {
    let mut days = Vec::with_capacity(25);
    let mut totals = Vec::with_capacity(25);

    for record in data.values() {
        if record.year != year {
            continue;
        }
        days.push(record.day);
        totals.push(record.total);
    }

    if days.is_empty() {
        return Ok(());
    }

    let total_time = totals.iter().sum::<f64>();
    let max_total = totals.iter().cloned().fold(0.0_f64, f64::max);
    let y_max = (max_total * 1.1).ceil();

    let filename = format!("benches/benches/advent_{}_bar_chart.png", year);
    let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Time taken for each day in {} ({:.2}ms total)", year, total_time),
            ("sans-serif", 24).into_font(),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (1u32..25u32).into_segmented(),
            0.0..y_max,
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Day")
        .y_desc("Time (ms)")
        .x_labels(25)
        .x_label_formatter(&|x| {
            if let SegmentValue::CenterOf(v) = x {
                format!("{}", v)
            } else {
                String::new()
            }
        })
        .axis_desc_style(("sans-serif", 16))
        .label_style(("sans-serif", 12))
        .draw()?;

    let bar_color = RGBColor(255, 127, 80);

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(bar_color.filled())
            .margin(2)
            .data(days.iter().zip(totals.iter()).map(|(&day, &total)| (day, total))),
    )?;

    root.present()?;

    Ok(())
}
