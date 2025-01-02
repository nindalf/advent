use std::path::PathBuf;

mod read;
mod write;

fn main() -> anyhow::Result<()> {
    // Read all files that match the glob patter /target/criterion/*/new/estimates.json
    let csv_path = PathBuf::from("benches/data.csv");

    let records = read::read_all_data(&csv_path)?;

    write::write_to_readme(&records)?;
    write::write_to_csv(&csv_path, records)?;

    Ok(())
}
