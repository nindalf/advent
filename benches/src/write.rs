use std::collections::BTreeMap;

pub struct Record {
    pub part_one_millis: f64,
    pub part_two_millis: f64,
    pub total: f64,
}

const README_TEMPLATE: &str = include_str!("readme.tmpl");

pub fn write_to_readme(data: &BTreeMap<(u32, u32), Record>) -> anyhow::Result<()> {
    let mut output = String::new();
    #[allow(clippy::single_element_loop)]
    for year in [2024] {
        output.push_str(&markdown_for_year(data, year));
    }

    let readme = README_TEMPLATE.replace("{{table}}", &output);
    std::fs::write("README.md", readme)?;

    Ok(())
}

fn markdown_for_year(data: &BTreeMap<(u32, u32), Record>, required_year: u32) -> String {
    let mut output = format!(
        "### {required_year}\n\n| Day  | Problem     | Solution    | Part 1 (ms) | Part 2 (ms) | Total (ms) |\n",
    );
    output.push_str(
        "|------|-------------|-------------|-------------|-------------|------------|\n",
    );
    let mut part_one_total = 0.0;
    let mut part_two_total = 0.0;
    let mut total_total = 0.0;
    for ((year, day), record) in data {
        if *year != required_year {
            continue;
        }

        let url = format!("https://adventofcode.com/{}/day/{}", year, day);
        let problem_name =
            get_problem_name(*year, *day).unwrap_or_else(|| "Unknown problem name".to_string());
        let solution_url = format!("/y{year}/src/day{day}/mod.rs");
        output.push_str(&format!(
            "| {day} | [{problem_name}]({url}) | [Solution]({solution_url}) | {:.2} | {:.2} | {:.2} |\n",
            record.part_one_millis, record.part_two_millis, record.total
        ));
        part_one_total += record.part_one_millis;
        part_two_total += record.part_two_millis;
        total_total += record.total;
    }

    output.push_str(&format!(
        "|  |  | Total | {:.2}ms | {:.2}ms | {:.2}ms |\n\n",
        part_one_total, part_two_total, total_total
    ));

    output
}

fn get_problem_name(year: u32, day: u32) -> Option<String> {
    let path = format!("y{year}/src/day{day}/Readme.md");
    let content = std::fs::read_to_string(path).unwrap();
    let re = regex::Regex::new(r"-- Day [0-9]+: (.*) --").unwrap();
    let problem_name = re.captures(&content)?.get(1)?.as_str().to_string();
    Some(problem_name)
}
